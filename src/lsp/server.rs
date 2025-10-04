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
