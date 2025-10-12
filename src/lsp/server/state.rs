use std::{collections::LinkedList, sync::mpsc};

use crate::lsp::{
    capabilities::client::ClientCapabilities,
    common::text_document::TextDocumentItem,
    notification::{ServerClientNotification, trace::TraceValue},
};

pub struct InitializedServerState {
    pub _client_capabilities: ClientCapabilities,
    pub is_client_initialized: bool,
    pub trace: TraceValue,
    pub notification_sender: mpsc::Sender<ServerClientNotification>,
    pub documents: Vec<TextDocumentItem>,
}
