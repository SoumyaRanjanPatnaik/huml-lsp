use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    text_document_sync: TextDocumentSyncOptions,
}

impl Default for ServerCapabilities {
    fn default() -> Self {
        Self {
            text_document_sync: TextDocumentSyncOptions {
                open_close: true,
                change: TextDocumentSyncKind::Incremental,
            },
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncOptions {
    open_close: bool,
    change: TextDocumentSyncKind,
}

#[derive(Serialize_repr, Debug)]
#[repr(u8)]
pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}
