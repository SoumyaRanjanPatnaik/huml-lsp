//! Defines the structures for Language Server Protocol (LSP) requests sent from the
//! client to the server.
//!
//! According to the LSP specification, a request is a message that requires a response.
//! This module defines the top-level `Request` container and an enumeration of all
//! supported request types (`RequestMethods`) along with their specific parameters.

/// structures and functionality related to initialize request
mod initialize;

use crate::rpc::Integer;
pub use initialize::*;
use serde::Deserialize;

/// Describes a request message sent from the client to the server.
///
/// Every processed [`Request`] must have a corresponding [Response] message sent
/// back to the client. The `id` field is used to correlate requests with responses.
///
/// See the [LSP specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#requestMessage)
/// for more details.
///
/// [Response]: crate::lsp::response::ResponseMessage
#[derive(Deserialize, Debug)]
pub struct Request {
    /// The unique identifier for the request, used to match it with a response.
    id: Integer,
    /// The specific method and parameters for this request.
    #[serde(flatten)]
    method: RequestMethods,
}

impl Request {
    /// Returns the unique identifier (`id`) of the request.
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Returns a reference to the enum that holds the specific method and parameters
    /// of this request.
    pub fn method(&self) -> &RequestMethods {
        &self.method
    }
}

/// An enumeration of all supported LSP request methods and their corresponding parameters.
///
/// This enum uses `serde` attributes to deserialize incoming JSON-RPC requests based on
/// the `method` field.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method", content = "params")]
pub enum RequestMethods {
    /// The `initialize` request is the first request sent from the client to the server.
    /// It is used to negotiate capabilities and initialize the server session.
    ///
    /// See the [specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#initialize)
    /// for more details.
    Initialize(InitializeParams),

    /// The `shutdown` request asks the server to shut down gracefully.
    /// The server should not exit until it receives an `exit` notification.
    /// No further requests should be processed after this one.
    ///
    /// See the [specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#shutdown)
    /// for more details.
    Shutdown,
}
