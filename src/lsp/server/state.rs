use std::sync::mpsc;

use crate::lsp::{
    notification::trace::TraceValue, request::ClientCapabilities, server::logger::LogEvent,
};

pub struct InitializedServerState {
    pub _client_capabilities: ClientCapabilities,
    pub is_client_initialized: bool,
    pub trace: TraceValue,
    pub log_event_sender: Option<mpsc::Sender<LogEvent>>,
}
