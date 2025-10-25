use serde::Deserialize;

use crate::lsp::common::text_document::TextDocumentIdentifier;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DidCloseTextDocumentParams<'a> {
    #[serde(borrow)]
    text_document: TextDocumentIdentifier<'a>,
}

impl<'a> DidCloseTextDocumentParams<'a> {
    pub fn text_document(&self) -> &TextDocumentIdentifier<'a> {
        &self.text_document
    }
}
