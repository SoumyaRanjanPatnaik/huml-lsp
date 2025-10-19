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

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClientServerNotification<'a> {
    #[serde(flatten)]
    #[serde(borrow)]
    variant: ClientServerNotificationVariant<'a>,

    #[serde(rename = "jsonrpc")]
    _jsonrpc: &'a str,
}

impl<'a> ClientServerNotification<'a> {
    pub fn into_variant(self) -> ClientServerNotificationVariant<'a> {
        self.variant
    }
}

/// Represents notifications sent from the client (e.g., editor) to the language server.
#[derive(Deserialize, Debug)]
#[serde(tag = "method", content = "params")]
pub enum ClientServerNotificationVariant<'a> {
    /// The `initialized` notification is sent from the client to the server after the client
    /// has received and successfully processed the [`Response Result`]
    /// It signals that the server can now send notifications and requests to the client.
    ///
    /// [`Response Result`]: crate::lsp::response::ResponseResult::Initialize
    #[serde(rename = "initialized")]
    Initialized(InitializedParams),

    /// The `$/setTrace` notification is sent from the client to the server to control the
    /// level of tracing and logging that the server should perform.
    #[serde(rename = "$/setTrace")]
    SetTrace(SetTraceParams),

    /// The document open notification is sent from the client to the server to signal
    /// newly opened text documents.
    #[serde(borrow)]
    #[serde(rename = "textDocument/didOpen")]
    DidOpen(DidOpenTextDocumentParams<'a>),

    /// The document open notification is sent from the client to the server to signal
    /// newly opened text documents.
    #[serde(borrow)]
    #[serde(rename = "textDocument/didChange")]
    DidChange(DidChangeTextDocumentParams<'a>),

    /// The `exit` notification is sent from the client to the server to ask it to exit.
    /// This notification must only be sent after a `shutdown` request has been successfully
    /// handled, transitioning the [Server] into the [Server::Shutdown] state.
    ///
    /// [Server]: crate::lsp::server::Server
    /// [Server::Shutdown]: crate::lsp::server::Server::Shutdown
    #[serde(rename = "exit")]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_deserialize_initialized_notification() {
        let json_input = r#"{
          "jsonrpc": "2.0",
          "method": "initialized",
          "params": {}
        }"#;

        let notification: ClientServerNotification = serde_json::from_str(json_input).unwrap();
        assert!(matches!(
            notification,
            ClientServerNotification {
                variant: ClientServerNotificationVariant::Initialized(..),
                _jsonrpc: "2.0"
            }
        ))
    }

    #[test]
    fn should_deserialize_set_trace() {
        let json_input = r#"{
          "jsonrpc": "2.0",
          "method": "$/setTrace",
          "params": {
            "value": "verbose"
          }
        }"#;

        let notification: ClientServerNotification = serde_json::from_str(json_input).unwrap();
        assert!(matches!(
            notification,
            ClientServerNotification {
                variant: ClientServerNotificationVariant::SetTrace(..),
                _jsonrpc: "2.0"
            }
        ))
    }

    #[test]
    fn should_deserialize_did_open() {
        let json_input = r#"{
          "jsonrpc": "2.0",
          "method": "textDocument/didOpen",
          "params": {
            "textDocument": {
              "languageId": "huml",
              "text": "hello world\n",
              "uri": "file:///tmp/test.huml",
              "version": 0
            }
          }
        }"#;

        let json_bytes = json_input.as_bytes();

        let notification: ClientServerNotification = serde_json::from_slice(json_bytes).unwrap();

        assert!(matches!(
            notification,
            ClientServerNotification {
                variant: ClientServerNotificationVariant::DidOpen(..),
                _jsonrpc: "2.0"
            }
        ));
    }

    #[test]
    fn should_deserialize_did_change() {
        let json_input = r#"{
          "jsonrpc": "2.0",
          "method": "textDocument/didChange",
          "params": {
            "contentChanges": [
              {
                "text": "hello world \n"
              }
            ],
            "textDocument": {
              "uri": "file:///tmp/test.huml",
              "version": 4
            }
          }
        }"#;

        let json_bytes = json_input.as_bytes();

        let notification: ClientServerNotification = serde_json::from_slice(json_bytes).unwrap();

        assert!(matches!(
            notification,
            ClientServerNotification {
                variant: ClientServerNotificationVariant::DidChange(..),
                _jsonrpc: "2.0"
            }
        ));
    }

    #[test]
    fn should_deserialize_exit_notification() {
        let json_input = r#"{
          "jsonrpc": "2.0",
          "method": "exit"
        }"#;

        let json_bytes = json_input.as_bytes();

        let notification: ClientServerNotification = serde_json::from_slice(json_bytes).unwrap();

        assert!(matches!(
            notification,
            ClientServerNotification {
                variant: ClientServerNotificationVariant::Exit,
                _jsonrpc: "2.0"
            }
        ));
    }
}
