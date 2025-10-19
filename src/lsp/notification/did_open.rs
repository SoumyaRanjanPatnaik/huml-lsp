use crate::lsp::common::text_document::{TextDocumentItem, TextDocumentItemOwned};
use serde::Deserialize;

/// Params for the [`textDocument/DidOpen`] notification
///
/// See [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#didOpenTextDocumentParams)
///
/// [`textDocument/DidOpen`]: crate::lsp::notification::ClientServerNotification::DidOpen
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenTextDocumentParams<'a> {
    #[serde(borrow)]
    text_document: TextDocumentItem<'a>,
}

impl<'a> DidOpenTextDocumentParams<'a> {
    pub fn text_document(&self) -> &TextDocumentItem<'_> {
        &self.text_document
    }

    pub fn into_text_document(self) -> TextDocumentItemOwned {
        return self.text_document.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import the parent module's items, including DidOpenTextDocumentParams
    use serde_json;

    #[test]
    fn should_deserialize_did_open_text_document_params() {
        // This JSON string mimics a real `textDocument/didOpen` notification's `params` object.
        // The top-level key is "textDocument" in camelCase.
        let json_input = r#"{
            "textDocument": {
                "uri": "file:///tmp/test.huml",
                "languageId": "huml",
                "version": 1,
                "text": "test_payload"
            }
        }"#;

        // Perform the deserialization.
        // serde infers the lifetime 'a from the `json_input` string slice.
        let params: DidOpenTextDocumentParams =
            serde_json::from_str(json_input).expect("Deserialization failed");

        // --- Assertions ---

        // 1. Verify the contents of the borrowed TextDocumentItem
        let text_document = params.text_document();
        assert_eq!(text_document.uri(), "file:///tmp/test.huml");
        assert_eq!(text_document.language_id(), "huml");
        assert_eq!(text_document.version(), 1);
        assert_eq!(text_document.text(), "test_payload");

        // 2. Verify that the `into_text_document` method works correctly
        // This consumes `params` and creates an owned version of the document.
        let owned_document: TextDocumentItemOwned = params.into_text_document();
        assert_eq!(owned_document.uri(), "file:///tmp/test.huml");
        assert_eq!(owned_document.language_id(), "huml");
        assert_eq!(owned_document.version(), 1);
    }

    #[test]
    fn should_fail_deserialization_on_missing_field() {
        // This JSON is invalid because it's missing the top-level "textDocument" key.
        let json_input = r#"{
            "uri": "file:///tmp/test.huml",
            "languageId": "huml",
            "version": 1,
            "text": "content"
        }"#;

        // We expect this to fail.
        let result = serde_json::from_str::<DidOpenTextDocumentParams>(json_input);
        assert!(
            result.is_err(),
            "Deserialization should fail for a malformed object"
        );
    }
}
