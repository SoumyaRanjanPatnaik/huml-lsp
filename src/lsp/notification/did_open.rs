use serde::Deserialize;

use crate::lsp::common::text_document::TextDocumentItem;

/// Params for the [`textDocument/DidOpen`] notification
///
/// See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#didOpenTextDocumentParams)
///
/// [`textDocument/DidOpen`]: crate::lsp::notification::ClientServerNotification::DidOpen
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenTextDocumentParams {
    text_document: TextDocumentItem,
}

impl DidOpenTextDocumentParams {
    pub fn text_document(&self) -> &TextDocumentItem {
        &self.text_document
    }

    pub fn into_text_document(self) -> TextDocumentItem {
        return self.text_document;
    }
}
