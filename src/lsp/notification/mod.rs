//! Defines the data structures for Language Server Protocol (LSP) notifications.
//!
//! Notifications are messages sent between the client and server that do not expect a
//! response. This module separates them into two main categories:
//! - [`ClientServerNotification`]: Notifications sent from the client to the server.
//! - [`ServerClientNotification`]: Notifications sent from the server to the client.

pub mod did_change;
pub mod did_open;
pub mod trace;

use crate::lsp::notification::{
    did_change::DidChangeTextDocumentParams,
    did_open::DidOpenTextDocumentParams,
    trace::{LogTraceParams, SetTraceParams},
};
use serde::{Deserialize, Serialize};

/// Represents notifications sent from the client (e.g., editor) to the language server.
#[derive(Deserialize, Debug)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "camelCase")]
pub enum ClientServerNotification {
    /// The `initialized` notification is sent from the client to the server after the client
    /// has received and successfully processed the [`Response Result`]
    /// It signals that the server can now send notifications and requests to the client.
    ///
    /// [`Response Result`]: crate::lsp::response::ResponseResult::Initialize
    Initialized(InitializedParams),

    /// The `$/setTrace` notification is sent from the client to the server to control the
    /// level of tracing and logging that the server should perform.
    #[serde(rename = "$/setTrace")]
    SetTrace(SetTraceParams),

    /// The document open notification is sent from the client to the server to signal
    /// newly opened text documents.
    #[serde(rename = "textDocument/didOpen")]
    DidOpen(DidOpenTextDocumentParams),

    /// The document open notification is sent from the client to the server to signal
    /// newly opened text documents.
    #[serde(rename = "textDocument/didChange")]
    DidChange(DidChangeTextDocumentParams),

    /// The `exit` notification is sent from the client to the server to ask it to exit.
    /// This notification must only be sent after a `shutdown` request has been successfully
    /// handled, transitioning the [Server] into the [Server::Shutdown] state.
    ///
    /// [Server]: crate::lsp::server::Server
    /// [Server::Shutdown]: crate::lsp::server::Server::Shutdown
    Exit,
}

/// The parameters for the `initialized` notification.
#[derive(Deserialize, Debug)]
pub struct InitializedParams {}

/// Represents notifications sent from the language server to the client.
#[derive(Serialize, Clone, Debug)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "camelCase")]
pub enum ServerClientNotification {
    /// The `$/logTrace` notification is sent from the server to the client to log
    /// diagnostic information. Its verbosity is controlled by the `$/setTrace` notification.
    #[serde(rename = "$/logTrace")]
    LogTrace(LogTraceParams),
}

/// A convenience implementation to easily convert `LogTraceParams` into a `ServerClientNotification`.
impl From<LogTraceParams> for ServerClientNotification {
    /// Converts [LogTraceParams] object to an instance of [ServerClientNotification::LogTrace]
    fn from(v: LogTraceParams) -> Self {
        Self::LogTrace(v)
    }
}
