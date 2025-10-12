use crate::rpc::{Integer, UInteger};
use serde::{Deserialize, Serialize};

/// An item to transfer a text document from the client to the server.
///
/// A text document is immutable
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentItem<'a> {
    /// The text document's URI.
    uri: &'a str,

    /// The text document's language identifier.
    language_id: &'a str,

    /// The version number of this document (it will increase after each
    /// change, including undo/redo).
    version: Integer,

    /// The content of the opened text document.
    text: &'a str,
}

impl<'a> TextDocumentItem<'a> {
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

/// An item to transfer a text document from the client to the server.
///
/// A text document is immutable
pub struct TextDocumentItemOwned {
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

impl TextDocumentItemOwned {
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

impl<'a> From<TextDocumentItem<'a>> for TextDocumentItemOwned {
    fn from(value: TextDocumentItem<'_>) -> Self {
        Self {
            uri: value.uri().to_owned(),
            language_id: value.language_id().to_owned(),
            version: value.version(),
            text: value.text.to_owned(),
        }
    }
}

/// Text documents are identified using a URI.
/// On the protocol level, URIs are passed as strings.
///
/// See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentIdentifier)
#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentIdentifier {
    uri: String,
}

impl TextDocumentIdentifier {
    pub fn uri(&self) -> &str {
        &self.uri
    }
}

/// An identifier to denote a specific version of a text document.
/// This information usually flows from the client to the server.
///
/// See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#versionedTextDocumentIdentifier)
#[derive(Serialize, Deserialize, Debug)]
pub struct VersionedTextDocumentIdentifier {
    #[serde(flatten)]
    identifier: TextDocumentIdentifier,
    version: Integer,
}

impl VersionedTextDocumentIdentifier {
    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn uri(&self) -> &str {
        self.identifier.uri()
    }
}

/// Indicates a position in the document
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Position {
    line: UInteger,
    character: UInteger,
}

impl Position {
    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn character(&self) -> u32 {
        self.character
    }
}

/// Indicates a range of text in the document
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Range {
    start: Position,
    end: Position,
}

impl Range {
    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }
}
