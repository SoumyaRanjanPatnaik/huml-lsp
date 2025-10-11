use crate::lsp::{
    error::ServerError,
    notification::{trace::{SetTraceParams, TraceValue}, Notification},
    request::{ClientCapabilities, InitializeParams, Request, RequestMethods},
    response::{initialize::InitializeResult, ResponseMessage, ResponsePayload, ResponseResult},
};

#[derive(Debug)]
pub enum Server {
    Uninitialized,
    Initialized {
        client_capabilities: ClientCapabilities,
        is_client_initialized: bool,
        trace: TraceValue
    },
    Shutdown,
}

impl Server {
    pub fn new() -> Self {
        Self::Uninitialized
    }

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
        *self = Server::Initialized {
            client_capabilities: params.capabilities().clone(),
            trace: TraceValue::Off,
            is_client_initialized: false,
        };
        InitializeResult::default().into()
    }

    fn handle_shutdown_req(&mut self) -> ResponsePayload {
        *self = Server::Shutdown;
        ResponsePayload::Result(ResponseResult::Shutdown)
    }

    fn handle_initialized_notification(&mut self) {
        match self {
            Server::Uninitialized => panic!(
                "Recieved initialized notification before the initialize request. Server not yet initialized"
            ),
            Server::Initialized {
                is_client_initialized,
                ..
            } => *is_client_initialized = false,
            _ => (),
        }
    }

    fn handle_set_trace(&mut self, params: SetTraceParams) {
        match self {
            Self::Initialized{ trace, .. } => *trace = *params.value(),
            _ => panic!(
                "Cannot set trace level when server not initialized"
            ),

        }
    }

    pub fn handle_request(&mut self, req: Request) -> Result<ResponseMessage, ServerError> {
        let response_payload = match req.method() {
            RequestMethods::Initialize(params) => self.handle_initialize_req(params),
            RequestMethods::Shutdown => self.handle_shutdown_req(),
        };
        Ok(ResponseMessage::new_for(req, response_payload))
    }

    pub fn handle_notification(&mut self, notification: Notification) -> Result<(), ServerError> {
        match notification {
            Notification::Initialized(_) => self.handle_initialized_notification(),
            Notification::SetTrace(params) =>
            Notification::Exit => process::exit(0),
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    use crate::lsp::{
        request::ClientCapabilities,
        response::{ResponsePayload, ResponseResult, initialize::InitializeResult},
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
            Server::Initialized {
                client_capabilities,
                is_client_initialized,
            } => {
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

        let mut server = Server::Initialized {
            client_capabilities: ClientCapabilities {},
            is_client_initialized: true,
        };

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
