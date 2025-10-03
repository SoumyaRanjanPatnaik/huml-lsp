use crate::lsp::{
    error::ServerError,
    request::{ClientCapabilities, InitializeParams, Request, RequestMethods},
    response::{ResponseMessage, ResponsePayload, initialize::InitializeResult},
};

#[derive(Debug)]
pub enum Server {
    Uninitialized,
    Initialized {
        client_capabilities: ClientCapabilities,
    },
}

impl Server {
    pub fn new() -> Self {
        Self::Uninitialized
    }

    /// Initialize the server
    fn initialize(&mut self, params: &InitializeParams) -> ResponsePayload {
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
        };
        InitializeResult::default().into()
    }

    pub fn handle_request(&mut self, req: Request) -> Result<ResponseMessage, ServerError> {
        let response_payload = match req.method() {
            RequestMethods::Initialize(params) => self.initialize(params),
        };
        Ok(ResponseMessage::new_for(req, response_payload))
    }
}
