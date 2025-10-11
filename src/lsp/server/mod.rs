mod logger;
mod state;

use std::{process, sync::mpsc};

use crate::lsp::{
    error::ServerError,
    notification::{
        ClientServerNotification,
        trace::{LogTraceParams, SetTraceParams, TraceValue},
    },
    request::{InitializeParams, Request, RequestMethods},
    response::{ResponseMessage, ResponsePayload, ResponseResult, initialize::InitializeResult},
    server::{
        logger::{LogEvent, initialize_logger},
        state::InitializedServerState,
    },
};

pub enum Server {
    Uninitialized,
    Initialized(InitializedServerState),
    Shutdown,
}

// Generic functions related to server
impl Server {
    pub fn new() -> Self {
        Self::Uninitialized
    }

    pub fn as_initialized(&self) -> Option<&InitializedServerState> {
        if let Self::Initialized(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_mut_initialized(&mut self) -> Option<&mut InitializedServerState> {
        if let Self::Initialized(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

// Request related methods
impl Server {
    /// Initialize the server
    fn handle_initialize_req(&mut self, params: &InitializeParams) -> ResponsePayload {
        use ResponsePayload::*;
        if matches!(self, Server::Initialized { .. }) {
            return Error {
                code: -1,
                message: "".to_string(),
                data: None,
            };
        }
        *self = Server::Initialized(InitializedServerState {
            _client_capabilities: params.capabilities().clone(),
            is_client_initialized: false,
            trace: TraceValue::Off,
            log_event_sender: None,
        });
        InitializeResult::default().into()
    }

    fn handle_shutdown_req(&mut self) -> ResponsePayload {
        self.shutdown_logger();
        *self = Server::Shutdown;
        ResponsePayload::Result(ResponseResult::Shutdown)
    }

    pub fn handle_request(&mut self, req: Request) -> Result<ResponseMessage, ServerError> {
        let response_payload = match req.method() {
            RequestMethods::Initialize(params) => self.handle_initialize_req(params),
            RequestMethods::Shutdown => self.handle_shutdown_req(),
        };
        Ok(ResponseMessage::new_for(req, response_payload))
    }
}

// Notification related methods
impl Server {
    fn handle_initialized_notification(&mut self) {
        match self {
            Server::Uninitialized => panic!(
                "Recieved initialized notification before the initialize request. Server not yet initialized"
            ),
            Server::Initialized(InitializedServerState {
                is_client_initialized,
                ..
            }) => *is_client_initialized = false,
            _ => (),
        }
    }

    fn handle_set_trace(&mut self, params: SetTraceParams) {
        match self {
            Self::Initialized(InitializedServerState {
                trace,
                log_event_sender,
                ..
            }) => {
                *trace = params.value();
                match log_event_sender {
                    Some(sender) => {
                        let _ = sender.send(trace.clone().into());
                    }
                    None if !trace.is_off() => {
                        log_event_sender.replace(initialize_logger(trace.clone()));
                    }
                    _ => (),
                }
            }
            _ => panic!("Cannot set trace level when server not initialized"),
        }
    }

    pub fn handle_notification(
        &mut self,
        notification: ClientServerNotification,
    ) -> Result<(), ServerError> {
        match notification {
            ClientServerNotification::Initialized(_) => self.handle_initialized_notification(),
            ClientServerNotification::SetTrace(params) => self.handle_set_trace(params),
            ClientServerNotification::Exit => process::exit(0),
        }
        Ok(())
    }
}

// Logging related functions
impl Server {
    fn send_log_event(&mut self, log_event: LogEvent) -> Result<(), mpsc::SendError<LogEvent>> {
        let state = self
            .as_mut_initialized()
            .expect("Server should be initialized to send events");

        let sender = match state.log_event_sender.as_ref() {
            Some(sender) => sender.clone(),
            None => match log_event {
                LogEvent::SetTrace(TraceValue::Off)
                // Herer
                | LogEvent::Shutdown => return Ok(()),
                _ => initialize_logger(state.trace),
            },
        };

        sender.send(log_event)
    }

    fn shutdown_logger(&mut self) {
        let _ = self.send_log_event(LogEvent::Shutdown);
    }

    fn _log_message(&mut self, message: String, verbose: Option<String>) {
        let _ = self.send_log_event(LogEvent::LogMessage(LogTraceParams::new(message, verbose)));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    use crate::lsp::{
        request::ClientCapabilities,
        response::{ResponsePayload, ResponseResult, initialize::InitializeResult},
        server::InitializedServerState,
    };

    #[test]
    fn should_initialize_server() {
        let mut server = Server::Uninitialized;
        let request = serde_json::from_value(json!({
            "id": 1,
            "method": "initialize",
            "params": {
                "capabilities": {}
            },
            "jsonrpc": "2.0"
        }))
        .unwrap();
        let response = server.handle_request(request).unwrap();
        match server {
            Server::Initialized(InitializedServerState {
                _client_capabilities: client_capabilities,
                is_client_initialized,
                ..
            }) => {
                assert_eq!(
                    is_client_initialized, false,
                    "Expected is_client_initialized to be false right after initialization"
                );

                assert_eq!(
                    client_capabilities,
                    serde_json::from_str("{}").unwrap(),
                    "Expected client_capabilities to match the value passed in the request"
                )
            }
            _ => assert!(false, "Expected the server to be initialized"),
        }

        assert_eq!(
            response.id(),
            1,
            "Expected response id to be same as request id "
        );

        assert!(
            matches!(
                response.payload(),
                ResponsePayload::Result(ResponseResult::Initialize(InitializeResult { .. }))
            ),
            "Expected response to contain an initialize result"
        );
    }

    #[test]
    fn test_shutdown() {
        let request = serde_json::from_value(json!({
            "id": 2,
            "method": "shutdown",
            "jsonrpc": "2.0"
        }))
        .unwrap();

        let mut server = Server::Initialized(InitializedServerState {
            _client_capabilities: ClientCapabilities {},
            is_client_initialized: true,
            log_event_sender: None,
            trace: TraceValue::Off,
        });

        let response = server.handle_request(request).unwrap();

        assert!(
            matches!(server, Server::Shutdown),
            "Expected server to be shutdown"
        );

        assert_eq!(
            response.id(),
            2,
            "Expected response id to be same as request id "
        );

        assert!(matches!(
            response.payload(),
            ResponsePayload::Result(ResponseResult::Shutdown)
        ));
    }
}
