pub mod initialize;

use crate::{
    lsp::{request::Request, response::initialize::InitializeResult},
    rpc::{Integer, LSPAny},
};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ResponseMessage {
    /// Contains the id of the request
    id: Integer,

    /// Contains the sucess or failure state of the payload
    #[serde(flatten)]
    payload: ResponsePayload,

    /// Always 2.0
    jsonrpc: String,
}

impl ResponseMessage {
    /// Create a new response message from the id of the request we're responding to
    /// and the payload
    pub unsafe fn new(request_id: Integer, payload: ResponsePayload) -> Self {
        Self {
            id: request_id,
            payload: payload,
            jsonrpc: "2.0".to_string(),
        }
    }

    /// Create a new response message from a request
    pub fn new_for(request: Request, payload: ResponsePayload) -> Self {
        Self {
            id: request.id(),
            payload: payload,
            jsonrpc: "2.0".to_string(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn payload(&self) -> &ResponsePayload {
        &self.payload
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ResponsePayload {
    Result(ResponseResult),
    Error {
        code: Integer,
        message: String,
        data: Option<LSPAny>,
    },
}

impl From<ResponseResult> for ResponsePayload {
    fn from(v: ResponseResult) -> Self {
        Self::Result(v)
    }
}

impl From<InitializeResult> for ResponsePayload {
    fn from(v: InitializeResult) -> Self {
        Self::Result(ResponseResult::Initialize(v))
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ResponseResult {
    Initialize(InitializeResult),
}
