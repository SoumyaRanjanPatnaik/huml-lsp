use crate::lsp::{capabilities::server::ServerCapabilities, properties::ServerInfo};
use serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct InitializeResult {
    capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    server_info: ServerInfo,
}
