use serde::Deserialize;

use crate::lsp::common::text_document::{Range, VersionedTextDocumentIdentifier};

/// Params for the [`textDocument/didChange`] notification
///
/// See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#didChangeTextDocumentParams)
///
/// [`textDocument/didChange`]: crate::lsp::notification::ClientServerNotification::DidChange
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeTextDocumentParams {
    text_document: VersionedTextDocumentIdentifier,
    content_changes: TextDocumentContentChangeEvent,
}

impl DidChangeTextDocumentParams {
    pub fn text_document(&self) -> &VersionedTextDocumentIdentifier {
        &self.text_document
    }

    pub fn content_changes(&self) -> &TextDocumentContentChangeEvent {
        &self.content_changes
    }
}

/// An event describing a change to a text document. If only a text is provided
///  it is considered to be the full content of the document.
///
///  See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentContentChangeEvent)
#[derive(Deserialize, Debug)]
pub struct TextDocumentContentChangeEvent {
    range: Option<Range>,
    text: String,
}

impl TextDocumentContentChangeEvent {
    pub fn range(&self) -> Option<Range> {
        self.range
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
