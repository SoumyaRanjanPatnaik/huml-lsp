use std::sync::mpsc;

use ouroboros::self_referencing;

use crate::lsp::{
    capabilities::client::ClientCapabilities,
    common::text_document::{Range, TextDocumentItemOwned},
    notification::{ServerClientNotification, trace::TraceValue},
};

pub struct InitializedServerState {
    pub _client_capabilities: ClientCapabilities,
    pub is_client_initialized: bool,
    pub trace: TraceValue,
    pub notification_sender: mpsc::Sender<ServerClientNotification>,
    pub documents: Vec<LineSeperatedDocument>,
}

#[self_referencing]
pub struct LineSeperatedDocument {
    pub full_document: TextDocumentItemOwned,
    #[borrows(full_document)]
    #[covariant]
    pub lines: Vec<&'this str>,
}

impl LineSeperatedDocument {
    pub fn into_full_document(self) -> TextDocumentItemOwned {
        self.into_heads().full_document
    }

    pub fn apply_diff_to_document(&self, range: Range, replace_with: &str) -> String {
        let (start_line, start_pos) = (range.start().line(), range.start().character());
        let (end_line, end_pos) = (range.end().line(), range.end().character());
        self.with_lines(|lines| {
            if start_line > lines.len() || end_line > lines.len() {
                panic!("Document out of sync. Ch&anges suggested outside range")
            }

            let before_start = &lines[..start_line];
            let stale_lines = &lines[start_line..=end_line];
            let after_end = &lines[(end_line + 1)..];

            let mut changed_region = String::new();

            // Add the unchanged bits from stale first line into
            if let Some(stale_line_first) = stale_lines.first() {
                changed_region.push_str(&stale_line_first[..start_pos]);
            }

            changed_region.push_str(replace_with);

            // Push unchanged bits fromo the stale last line into the updated last line
            if let Some(stale_line_last) = stale_lines.first() {
                changed_region.push_str(&stale_line_last[end_pos..]);
            }

            // Combine the channged and the unchanged parts of the documeent
            let updated_document = [before_start, &[&changed_region], after_end]
                .concat()
                .join("\r\n");

            updated_document
        })
    }
}

impl From<TextDocumentItemOwned> for LineSeperatedDocument {
    fn from(value: TextDocumentItemOwned) -> Self {
        LineSeperatedDocumentBuilder {
            full_document: value,
            lines_builder: |document| document.text().lines().collect(),
        }
        .build()
    }
}
