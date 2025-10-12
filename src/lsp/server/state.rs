use std::sync::mpsc;

use crate::lsp::{
    capabilities::ClientCapabilities,
    notification::{ServerClientNotification, trace::TraceValue},
};

pub struct InitializedServerState {
    pub _client_capabilities: ClientCapabilities,
    pub is_client_initialized: bool,
    pub trace: TraceValue,
    pub notification_sender: mpsc::Sender<ServerClientNotification>,
}
