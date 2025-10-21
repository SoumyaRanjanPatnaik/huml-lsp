use std::borrow::Cow;

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
    #[serde(borrow)]
    text: Cow<'a, str>,
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
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
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
    pub const fn new(uri: String, language_id: String, version: Integer, text: String) -> Self {
        Self {
            uri,
            language_id,
            version,
            text,
        }
    }

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

    pub fn as_parts(&self) -> (&str, &str, i32, &str) {
        (self.uri(), self.language_id(), self.version(), self.text())
    }

    pub fn into_parts(self) -> (String, String, i32, String) {
        (self.uri, self.language_id, self.version, self.text)
    }
}

impl<'a> From<TextDocumentItem<'a>> for TextDocumentItemOwned {
    fn from(value: TextDocumentItem<'_>) -> Self {
        Self {
            uri: value.uri().to_owned(),
            language_id: value.language_id().to_owned(),
            version: value.version(),
            text: value.text.to_string(),
        }
    }
}

/// Text documents are identified using a URI.
/// On the protocol level, URIs are passed as strings.
///
/// See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentIdentifier)
#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentIdentifier<'a> {
    uri: &'a str,
}

impl<'a> TextDocumentIdentifier<'a> {
    pub fn uri(&self) -> &str {
        &self.uri
    }
}

/// An identifier to denote a specific version of a text document.
/// This information usually flows from the client to the server.
///
/// See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#versionedTextDocumentIdentifier)
#[derive(Serialize, Deserialize, Debug)]
pub struct VersionedTextDocumentIdentifier<'a> {
    #[serde(flatten)]
    #[serde(borrow)]
    identifier: TextDocumentIdentifier<'a>,
    version: Integer,
}

impl<'a> VersionedTextDocumentIdentifier<'a> {
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
    pub fn new(line: UInteger, character: UInteger) -> Self {
        Self { line, character }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn character(&self) -> usize {
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
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module.
    use serde_json;

    #[test]
    fn should_deserialize_text_document_item() {
        let json_input = r#"{
            "uri": "file:///path/to/file.txt",
            "languageId": "plaintext",
            "version": 7,
            "text": "Hello, world!"
        }"#;

        let deserialized: TextDocumentItem =
            serde_json::from_str(json_input).expect("Deserialization failed");

        assert_eq!(deserialized.uri(), "file:///path/to/file.txt");
        assert_eq!(deserialized.language_id(), "plaintext");
        assert_eq!(deserialized.version(), 7);
        assert_eq!(deserialized.text(), "Hello, world!");
    }

    #[test]
    fn should_deserialize_text_document_item_owned() {
        let json_input = r#"{
            "uri": "file:///path/to/another_file.rs",
            "languageId": "rust",
            "version": 42,
            "text": "fn main() {}"
        }"#;

        let deserialized: TextDocumentItemOwned =
            serde_json::from_str(json_input).expect("Deserialization failed");

        assert_eq!(deserialized.uri(), "file:///path/to/another_file.rs");
        assert_eq!(deserialized.language_id(), "rust");
        assert_eq!(deserialized.version(), 42);
        assert_eq!(deserialized.text(), "fn main() {}");
    }

    #[test]
    fn should_deserialize_text_document_identifier() {
        let json_input = r#"{
            "uri": "file:///path/to/doc.huml"
        }"#;

        let deserialized: TextDocumentIdentifier =
            serde_json::from_str(json_input).expect("Deserialization failed");

        assert_eq!(deserialized.uri(), "file:///path/to/doc.huml");
    }

    #[test]
    fn should_deserialize_versioned_text_document_identifier() {
        // Note: The `version` field is at the same level as `uri` due to `#[serde(flatten)]`
        let json_input = r#"{
            "uri": "file:///path/to/versioned_doc.json",
            "version": 12
        }"#;

        let deserialized: VersionedTextDocumentIdentifier =
            serde_json::from_str(json_input).expect("Deserialization failed");

        assert_eq!(deserialized.uri(), "file:///path/to/versioned_doc.json");
        assert_eq!(deserialized.version(), 12);
    }

    #[test]
    fn should_deserialize_position() {
        let json_input = r#"{
            "line": 10,
            "character": 25
        }"#;

        let deserialized: Position =
            serde_json::from_str(json_input).expect("Deserialization failed");

        assert_eq!(deserialized.line(), 10);
        assert_eq!(deserialized.character(), 25);
    }

    #[test]
    fn should_deserialize_range() {
        let json_input = r#"{
            "start": { "line": 5, "character": 10 },
            "end": { "line": 5, "character": 20 }
        }"#;

        let deserialized: Range = serde_json::from_str(json_input).expect("Deserialization failed");

        assert_eq!(deserialized.start().line(), 5);
        assert_eq!(deserialized.start().character(), 10);
        assert_eq!(deserialized.end().line(), 5);
        assert_eq!(deserialized.end().character(), 20);
    }
}
