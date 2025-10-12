use std::sync::mpsc;

use crate::lsp::{
    capabilities::client::ClientCapabilities,
    common::text_document::TextDocumentItemOwned,
    notification::{ServerClientNotification, trace::TraceValue},
};

pub struct InitializedServerState {
    pub _client_capabilities: ClientCapabilities,
    pub is_client_initialized: bool,
    pub trace: TraceValue,
    pub notification_sender: mpsc::Sender<ServerClientNotification>,
    pub documents: Vec<LineSeperatedDocument>,
}

pub struct LineSeperatedDocument {
    full_document: TextDocumentItemOwned,
}

impl LineSeperatedDocument {
    pub fn full_document(&self) -> &TextDocumentItemOwned {
        &self.full_document
    }
}
