use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    #[serde(default)]
    text_document: Option<TextDocumentClientCapabilities>,
}

impl ClientCapabilities {
    pub fn text_document(&self) -> Option<&TextDocumentClientCapabilities> {
        self.text_document.as_ref()
    }
}

/// Text document specific client capabilities.
///
/// See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentClientCapabilities) for more info.

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TextDocumentClientCapabilities {
    synchronization: Option<TextDocumentSyncClientCapabilities>,
}

/// Represents the synchronization capabilities supported by the client
///
/// See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentSyncClientCapabilities) for more info
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncClientCapabilities {
    /// Whether text document synchronization supports dynamic registration.
    #[serde(default)]
    dynamic_registration: bool,

    /// The client supports sending will save notifications.
    #[serde(default)]
    will_save: bool,

    /// The client supports sending a will save request and
    /// waits for a response providing text edits which will
    /// be applied to the document before it is saved.
    #[serde(default)]
    will_save_wait_until: bool,

    /// The client supports did save notifications.
    #[serde(default)]
    did_save: bool,
}
