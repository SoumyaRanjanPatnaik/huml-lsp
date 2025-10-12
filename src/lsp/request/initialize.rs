use crate::{lsp::capabilities::ClientCapabilities, rpc::Integer};
use serde::Deserialize;

/// Params for a [super::RequestMethod::Initialize]
/// [Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#initializeParams)
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    /// The process Id of the parent process that started the server. Is null if
    /// the process has not been started by another process. If the parent
    /// process is not alive then the server should exit (see exit notification)
    /// its process.
    process_id: Option<Integer>,

    /// Information about the client
    client_info: Option<ClientInfo>,

    /// The capabilities provided by the client (editor or tool)
    capabilities: ClientCapabilities,

    /// The workspace folders configured in the client when the server starts.
    ///	This property is only available if the client supports workspace folders.
    ///	It can be `null` if the client supports workspace folders but none are
    ///	configured.
    workspace_folders: Option<WorkspaceFolder>,
}

impl InitializeParams {
    pub fn process_id(&self) -> Option<i32> {
        self.process_id
    }

    pub fn client_info(&self) -> Option<&ClientInfo> {
        self.client_info.as_ref()
    }

    pub fn capabilities(&self) -> &ClientCapabilities {
        &self.capabilities
    }

    pub fn workspace_folders(&self) -> Option<&WorkspaceFolder> {
        self.workspace_folders.as_ref()
    }
}

/// Information about the client
///
/// @since 3.15.0
#[derive(Deserialize, Debug)]
pub struct ClientInfo {
    name: String,
    version: String,
}

impl ClientInfo {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

#[derive(Deserialize, Debug)]
pub struct WorkspaceFolder {
    /// The associated URI for this workspace folder.
    uri: String,

    /// The name of the workspace folder. Used to refer to this
    ///  workspace folder in the user interface.
    name: String,
}

impl WorkspaceFolder {
    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
