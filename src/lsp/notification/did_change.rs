use std::borrow::Cow;

use serde::Deserialize;

use crate::lsp::common::text_document::{Range, VersionedTextDocumentIdentifier};

/// Params for the [`textDocument/didChange`] notification
///
/// See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#didChangeTextDocumentParams)
///
/// [`textDocument/didChange`]: crate::lsp::notification::ClientServerNotification::DidChange
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeTextDocumentParams<'a> {
    #[serde(borrow)]
    text_document: VersionedTextDocumentIdentifier<'a>,
    #[serde(borrow)]
    content_changes: Vec<TextDocumentContentChangeEvent<'a>>,
}

impl<'a> DidChangeTextDocumentParams<'a> {
    pub fn text_document(&self) -> &VersionedTextDocumentIdentifier<'_> {
        &self.text_document
    }

    pub fn content_changes(&self) -> &Vec<TextDocumentContentChangeEvent<'_>> {
        &self.content_changes
    }
}

/// An event describing a change to a text document. If only a text is provided
///  it is considered to be the full content of the document.
///
///  See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentContentChangeEvent)
#[derive(Deserialize, Debug)]
pub struct TextDocumentContentChangeEvent<'a> {
    range: Option<Range>,
    #[serde(borrow)]
    text: Cow<'a, str>,
}

impl<'a> TextDocumentContentChangeEvent<'a> {
    pub fn range(&self) -> Option<Range> {
        self.range
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
