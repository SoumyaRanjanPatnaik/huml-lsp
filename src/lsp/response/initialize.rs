use crate::lsp::properties::{ServerCapabilities, ServerInfo};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct InitializeResult {
    capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    server_info: ServerInfo,
}
