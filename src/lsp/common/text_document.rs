use serde::Deserialize;

use crate::rpc::Integer;

/// An item to transfer a text document from the client to the server.
///
/// A text document is immutable
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem {
    /// The text document's URI.
    uri: String,

    /// The text document's language identifier.
    language_id: String,

    /// The version number of this document (it will increase after each
    /// change, including undo/redo).
    version: Integer,

    /// The content of the opened text document.
    text: String,
}

impl TextDocumentItem {
    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn language_id(&self) -> &str {
        &self.language_id
    }

    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
