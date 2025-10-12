//! Defines the structures for Language Server Protocol (LSP) responses sent from the
//! server back to the client.
//!
//! For every request a client sends, the server must send a corresponding `ResponseMessage`.
//! This module provides the necessary structures to build both successful responses,
//! which contain a `result`, and error responses, which contain an `error` object.

pub mod initialize;

use crate::{
    lsp::{request::Request, response::initialize::InitializeResult},
    rpc::{Integer, LSPAny},
};
use serde::Serialize;

/// Represents a complete response message to be sent to the client.
///
/// It correlates with a specific [`Request`] via the `id` field and contains either
/// a successful result or an error.
///
/// See the [LSP specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#responseMessage)
/// for more details.
#[derive(Serialize, Debug)]
pub struct ResponseMessage {
    /// The ID of the request that this response is for.
    id: Integer,

    /// The payload of the response, containing either a `Result` or an `Error`.
    #[serde(flatten)]
    payload: ResponsePayload,

    /// The JSON-RPC version, always "2.0".
    jsonrpc: String,
}

impl ResponseMessage {
    /// Creates a new `ResponseMessage` with a specified request ID and payload.
    ///
    /// # Unsafe
    /// This function is marked `unsafe` because it allows the creation of a response
    /// with an arbitrary ID, which could potentially violate the LSP specification if
    /// the ID does not correspond to a pending request from the client. It should be
    /// used with caution.
    pub unsafe fn new(request_id: Integer, payload: ResponsePayload) -> Self {
        Self {
            id: request_id,
            payload,
            jsonrpc: "2.0".to_string(),
        }
    }

    /// Creates a new `ResponseMessage` directly from a `Request` object.
    ///
    /// This is the preferred, safe method for creating a response, as it ensures
    /// that the response ID correctly matches the request ID.
    pub fn new_for(request: Request, payload: ResponsePayload) -> Self {
        Self {
            id: request.id(),
            payload,
            jsonrpc: "2.0".to_string(),
        }
    }

    /// Returns the ID of the request this message is responding to.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Returns a reference to the payload of the response.
    pub fn payload(&self) -> &ResponsePayload {
        &self.payload
    }
}

/// An enum representing the body of a `ResponseMessage`.
/// It can either be a successful `Result` or an `Error`.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ResponsePayload {
    /// Represents a successful request execution, containing the result data.
    Result(ResponseResult),
    /// Represents a failed request execution, containing error details.
    Error {
        /// A number indicating the error type that occurred.
        code: Integer,
        /// A string providing a short description of the error.
        message: String,
        /// A primitive or structured value that contains additional
        /// information about the error. Can be omitted.
        data: Option<LSPAny>,
    },
}

/// A convenience implementation to easily wrap a `ResponseResult` in a `ResponsePayload`.
impl From<ResponseResult> for ResponsePayload {
    fn from(v: ResponseResult) -> Self {
        Self::Result(v)
    }
}

/// A convenience implementation to wrap an `InitializeResult` directly into a `ResponsePayload`.
impl From<InitializeResult> for ResponsePayload {
    fn from(v: InitializeResult) -> Self {
        Self::Result(ResponseResult::Initialize(v))
    }
}

/// An enumeration of all possible successful response types.
///
/// Each variant corresponds to the successful result of a specific request method.
/// The `untagged` attribute means that `serde` will serialize the variant's content
/// directly, without wrapping it in an object with the variant's name.
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ResponseResult {
    /// The result of a successful `initialize` request.
    Initialize(InitializeResult),
    /// The result of a successful `shutdown` request, which is `null` in JSON.
    Shutdown,
}
