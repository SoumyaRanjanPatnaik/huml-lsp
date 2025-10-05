use crate::lsp::{
    error::ServerError,
    notification::Notification,
    request::{ClientCapabilities, InitializeParams, Request, RequestMethods},
    response::{ResponseMessage, ResponsePayload, initialize::InitializeResult},
};

#[derive(Debug)]
pub enum Server {
    Uninitialized,
    Initialized {
        client_capabilities: ClientCapabilities,
        is_client_initialized: bool,
    },
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
            is_client_initialized: false,
        };
        InitializeResult::default().into()
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
        }
    }

    pub fn handle_request(&mut self, req: Request) -> Result<ResponseMessage, ServerError> {
        let response_payload = match req.method() {
            RequestMethods::Initialize(params) => self.handle_initialize_req(params),
        };
        Ok(ResponseMessage::new_for(req, response_payload))
    }

    pub fn handle_notification(&mut self, notification: Notification) -> Result<(), ServerError> {
        match notification {
            Notification::Initialized(_) => self.handle_initialized_notification(),
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::lsp::{
        response::{ResponsePayload, ResponseResult, initialize::InitializeResult},
        server::Server,
    };

    #[test]
    fn should_initialize_server() {
        let mut server = Server::Uninitialized;
        let request = serde_json::from_value(json!({
            "id": 1,
            "method": "initialize",
            "params": {
                "capabilities": {}
            }
        }))
        .unwrap();
        let response = server.handle_request(request).unwrap();
        match server {
            Server::Uninitialized => assert!(false, "Expected the server to be initialized"),
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
}
