//! Implements the main language server logic, state management, and message dispatch.
//!
//! This module contains the core `Server` enum, which acts as a state machine
//! representing the server's lifecycle (Uninitialized, Initialized, Shutdown). It is
//! responsible for receiving requests and notifications, dispatching them to the
//! appropriate handlers, and managing the server's state accordingly.

mod state;
mod writer;

use crate::lsp::{
    common::text_document::TextDocumentItemOwned,
    error::ServerError,
    notification::{
        ClientServerNotification, ClientServerNotificationVariant,
        did_change::DidChangeTextDocumentParams,
        did_open::DidOpenTextDocumentParams,
        trace::{LogTraceParams, SetTraceParams, TraceValue},
    },
    request::{InitializeParams, Request, RequestMethod},
    response::{ResponseMessage, ResponsePayload, ResponseResult, initialize::InitializeResult},
    server::{
        state::{InitializedServerState, LineSeperatedDocument},
        writer::initialize_notification_loop,
    },
};
use std::{
    io::{Write, stdout},
    process,
};

/// Represents the state of the language server throughout its lifecycle.
///
/// The server transitions through these states based on the LSP lifecycle messages
/// it receives from the client (e.g., `initialize`, `initialized`, `shutdown`, `exit`).
pub enum Server {
    /// The initial state of the server before the `initialize` request is received.
    /// In this state, the server can only respond to the `initialize` request.
    Uninitialized,
    /// The state after the server has successfully responded to an `initialize` request.
    /// It holds the server's state, including client capabilities and trace settings.
    Initialized(InitializedServerState),
    /// The state after the server has received a `shutdown` request.
    /// In this state, most requests and notifications will be ignored, and the server
    /// is waiting for an `exit` notification to terminate.
    Shutdown,
}

// Generic functions related to server
impl Server {
    /// Creates a new server in the `Uninitialized` state.
    pub fn new() -> Self {
        Self::Uninitialized
    }

    /// Returns an immutable reference to the initialized server state, if available.
    ///
    /// Returns `Some(&InitializedServerState)` if the server is in the `Initialized` state,
    /// otherwise returns `None`.
    pub fn as_initialized(&self) -> Option<&InitializedServerState> {
        if let Self::Initialized(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the initialized server state, if available.
    ///
    /// Returns `Some(&mut InitializedServerState)` if the server is in the `Initialized` state,
    /// otherwise returns `None`.
    pub fn as_mut_initialized(&mut self) -> Option<&mut InitializedServerState> {
        if let Self::Initialized(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the server is [`Initialized`].
    ///
    /// [`Initialized`]: Server::Initialized
    #[must_use]
    pub fn is_initialized(&self) -> bool {
        matches!(self, Self::Initialized(..))
    }
}

// Request related methods
impl Server {
    /// Handles the `initialize` request from the client.
    ///
    /// This method transitions the server from the `Uninitialized` state to the `Initialized`
    /// state. It sets up the notification writer, stores client capabilities, and prepares
    /// the server for further communication. It returns an error if called more than once.
    fn handle_initialize_req(&mut self, params: &InitializeParams) -> ResponsePayload {
        use ResponsePayload::*;
        if matches!(self, Server::Initialized { .. }) {
            return Error {
                // TODO: Standardize errors
                // As per LSP spec: "Using the initialize request a second time..."
                // "...is diagnosed as an error with a predefined code."
                code: -32002, // ServerErrorCodes::ServerNotInitialized
                message: "Server is already initialized".to_string(),
                data: None,
            };
        }

        // Initialize notification writer
        let notification_sender =
            initialize_notification_loop(|msg| write!(stdout().lock(), "{msg}"));

        *self = Server::Initialized(InitializedServerState {
            _client_capabilities: params.capabilities().clone(),
            is_client_initialized: false,
            trace: TraceValue::Off,
            notification_sender,
            documents: vec![],
        });

        self.log_message(
            "Server initialized. Waiting for client initialized ack".to_string(),
            None,
        );

        InitializeResult::default().into()
    }

    /// Handles the `shutdown` request from the client.
    ///
    /// This method transitions the server to the `Shutdown` state, preparing it
    /// to terminate upon receiving an `exit` notification.
    fn handle_shutdown_req(&mut self) -> ResponsePayload {
        *self = Server::Shutdown;
        ResponsePayload::Result(ResponseResult::Shutdown)
    }

    /// The main entry point for dispatching all incoming requests from the client.
    ///
    /// It takes a `Request` and routes it to the appropriate handler based on its method.
    /// It returns a `ResponseMessage` to be sent back to the client.
    pub fn handle_request<'a>(&mut self, req: &'a Request) -> Result<ResponseMessage, ServerError> {
        let response_payload = match req.method() {
            RequestMethod::Initialize(params) => self.handle_initialize_req(params),
            RequestMethod::Shutdown => self.handle_shutdown_req(),
        };
        Ok(ResponseMessage::new_for(req, response_payload))
    }
}

// Notification related methods
impl Server {
    /// Handles the `initialized` notification from the client.
    ///
    /// This notification confirms that the client has successfully processed the
    /// `initialize` response.
    fn handle_initialized_notification(&mut self) {
        match self {
            Server::Uninitialized => panic!(
                "Received initialized notification before the initialize request. Server not yet initialized"
            ),
            Server::Initialized(InitializedServerState {
                is_client_initialized,
                ..
            }) => *is_client_initialized = false,
            _ => (),
        }
    }

    /// Handles the [`$/setTrace`] notification to adjust the server's logging verbosity.
    ///
    /// [`$/setTrace`]: crate::lsp::notification::ClientServerNotification::SetTrace
    fn handle_set_trace(&mut self, params: SetTraceParams) {
        match self {
            Self::Initialized(InitializedServerState { trace, .. }) => {
                *trace = params.value();
            }
            _ => panic!("Cannot set trace level when server not initialized"),
        }
    }

    /// Handles the `textDocument/didOpen` notification
    pub fn handle_did_open(&mut self, params: DidOpenTextDocumentParams) {
        let opened_document_item: TextDocumentItemOwned = params.into_text_document();

        let opened_document_uri = opened_document_item.uri().to_string();
        let log_verbose = format!("{:?}", opened_document_item);
        let log_message = format!("Opening document {opened_document_uri}");
        self.log_message(log_message, Some(log_verbose));

        match self {
            Self::Initialized(InitializedServerState { documents, .. }) => {
                // Replace document if already exists
                let existing_doc_position = documents
                    .iter()
                    .position(|doc| doc.borrow_full_document().uri() == opened_document_item.uri());

                let line_seperated_docuemnt = LineSeperatedDocument::from(opened_document_item);
                match existing_doc_position {
                    Some(idx) => documents[idx] = line_seperated_docuemnt,
                    None => documents.push(line_seperated_docuemnt),
                };
            }
            _ => panic!("Cannot handle text document notifications when server not initialized"),
        }
    }

    /// Handles the `textDocument/didChange` notification
    pub fn handle_did_change(&mut self, params: DidChangeTextDocumentParams) {
        let InitializedServerState { documents, .. } = self
            .as_mut_initialized()
            .expect("Cannot handle text document notifications when server not initialized");

        // Update document if exists
        let Some(document_lines) = documents
            .iter_mut()
            .find(|doc| doc.borrow_full_document().uri() == params.text_document().uri())
        else {
            return;
        };

        // Metadata required for constructing the new TextDocumentItemOwned object
        let (uri, language_id, ..) = document_lines.borrow_full_document().clone().into_parts();
        let updated_version = params.text_document().version();
        // let text_changes_recieved = params.content_changes().text();
        //
        // // Get the range of text changed
        // let Some(range) = params.content_changes().range() else {
        //     // Handle full document update if range is None
        //     let updated_full_document = TextDocumentItemOwned::new(
        //         uri.to_string(),
        //         language_id.to_string(),
        //         updated_version,
        //         text_changes_recieved.to_string(),
        //     );
        //     *document_lines = LineSeperatedDocument::from(updated_full_document);
        //     return;
        // };

        let change_diff: Vec<_> = params
            .content_changes()
            .iter()
            .filter_map(|change| {
                let range_opt = change.range();
                let text = change.text();
                range_opt.map(|range| (range, text))
            })
            .collect();

        let diff_applied_text_document = document_lines.apply_diff_to_document(&change_diff);

        let updated_text_document_item = TextDocumentItemOwned::new(
            uri.to_string(),
            language_id.to_string(),
            updated_version,
            diff_applied_text_document,
        );
        *document_lines = LineSeperatedDocument::from(updated_text_document_item)
    }

    /// The main entry point for dispatching all incoming notifications from the client.
    ///
    /// It takes a `ClientServerNotification` and routes it to the appropriate handler.
    pub fn handle_notification(
        &mut self,
        notification: ClientServerNotification,
    ) -> Result<(), ServerError> {
        match notification.into_variant() {
            ClientServerNotificationVariant::Initialized(_) => {
                self.handle_initialized_notification()
            }
            ClientServerNotificationVariant::Exit => process::exit(0),
            ClientServerNotificationVariant::SetTrace(params) => self.handle_set_trace(params),

            // Text Document Related Notifications
            ClientServerNotificationVariant::DidChange(params) => self.handle_did_change(params),
            ClientServerNotificationVariant::DidOpen(document_sync) => {
                self.handle_did_open(document_sync)
            }
        }
        Ok(())
    }

    /// Sends a [`$/logTrace`] notification to the client if tracing is enabled.
    ///
    /// The verbosity of the message is determined by the current `TraceValue`
    /// set by the client.
    ///
    /// [`$/logTrace`]: crate::lsp::notification::ServerClientNotification::LogTrace
    fn log_message(&mut self, message: String, verbose: Option<String>) {
        let state = self
            .as_mut_initialized()
            .expect("Logging shouldn't happen if the server is not initialized");

        writeln!(std::io::stderr(), "Sending log").unwrap();
        // let log_params = match state.trace {
        //     TraceValue::Off => return,
        //     TraceValue::Message => LogTraceParams::new(message, None),
        //     TraceValue::Verbose => LogTraceParams::new(message, verbose),
        // };
        let log_params = LogTraceParams::new(message, verbose);
        let _ = state
            .notification_sender
            .send(log_params.into())
            .expect("Notification send failed");
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc;

    use super::*;
    use serde_json::json;

    use crate::lsp::{
        capabilities::client::ClientCapabilities,
        response::{ResponsePayload, ResponseResult, initialize::InitializeResult},
        server::InitializedServerState,
    };

    #[test]
    fn should_initialize_server() {
        let mut server = Server::Uninitialized;
        let request_str = serde_json::to_string(&json!({
            "id": 1,
            "method": "initialize",
            "params": {
                "capabilities": {}
            },
            "jsonrpc": "2.0"
        }))
        .unwrap();
        let request: Request<'_> = serde_json::from_str(&request_str).unwrap();
        let response = server.handle_request(&request).unwrap();
        match server {
            Server::Initialized(InitializedServerState {
                _client_capabilities: client_capabilities,
                is_client_initialized,
                ..
            }) => {
                assert_eq!(
                    is_client_initialized, false,
                    "Expected is_client_initialized to be false right after initialization"
                );

                let actual_capabilities_str = serde_json::to_string(&client_capabilities).unwrap();

                let expected_capabilities_str =
                    serde_json::to_string(&ClientCapabilities::default()).unwrap();

                assert_eq!(
                    expected_capabilities_str, actual_capabilities_str,
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
        let request_str = serde_json::to_string(&json!({
            "id": 2,
            "method": "shutdown",
            "jsonrpc": "2.0"
        }))
        .unwrap();
        let request = serde_json::from_str(&request_str).unwrap();

        let (notification_sender, _notification_reciever) = mpsc::channel();
        let mut server = Server::Initialized(InitializedServerState {
            _client_capabilities: ClientCapabilities::default(),
            is_client_initialized: true,
            notification_sender: notification_sender,
            trace: TraceValue::Off,
            documents: vec![],
        });

        let response = server.handle_request(&request).unwrap();

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
