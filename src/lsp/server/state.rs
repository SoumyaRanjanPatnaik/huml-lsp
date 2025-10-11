use std::sync::mpsc;

use crate::lsp::{
    notification::{ServerClientNotification, trace::TraceValue},
    request::ClientCapabilities,
};

pub struct InitializedServerState {
    pub _client_capabilities: ClientCapabilities,
    pub is_client_initialized: bool,
    pub trace: TraceValue,
    pub notification_sender: mpsc::Sender<ServerClientNotification>,
}
