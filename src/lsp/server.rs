use crate::lsp::{error::ServerError, request::Request, response::ResponseMessage};

#[derive(Debug)]
pub enum Server {
    Uninitialized,
    Initialized {},
}

impl Server {
    pub fn new() -> Self {
        Self::Uninitialized
    }

    pub fn handle_request(&self, _req: Request) -> Result<ResponseMessage, ServerError> {
        match self {
            Server::Uninitialized => unimplemented!(),
            Server::Initialized {} => unimplemented!(),
        }
    }
}
