use std::{collections::LinkedList, sync::mpsc};

use ouroboros::self_referencing;

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

#[self_referencing]
pub struct LineSeperatedDocument {
    pub full_document: TextDocumentItemOwned,
    #[borrows(full_document)]
    #[covariant]
    lines: LinkedList<&'this str>,
}

impl From<TextDocumentItemOwned> for LineSeperatedDocument {
    fn from(value: TextDocumentItemOwned) -> Self {
        LineSeperatedDocumentBuilder {
            full_document: value,
            lines_builder: |document| document.text().lines().collect(),
        }
        .build()
    }
}
