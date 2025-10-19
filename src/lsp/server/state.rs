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

    pub fn apply_diff_to_document(&self, diff: &[(Range, &str)]) -> String {
        let mut document = String::new();
        for (range, replace_with) in diff {
            let (start_line, start_pos) = (range.start().line(), range.start().character());
            let (end_line, end_pos) = (range.end().line(), range.end().character());
            document = self.with_lines(|lines| {
                if start_line > lines.len() || end_line > lines.len() {
                    panic!("Document out of sync. Changes suggested outside range")
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
                if let Some(stale_line_last) = stale_lines.last() {
                    changed_region.push_str(&stale_line_last[end_pos..]);
                }

                // Combine the channged and the unchanged parts of the documeent
                let updated_document = [before_start, &[&changed_region], after_end]
                    .concat()
                    .join("\n");

                updated_document
            })
        }
        document
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

#[cfg(test)]
mod tests {
    use crate::lsp::common::text_document::Position;

    use super::*;

    const TEST_TEXT: &str = r#"Hello, I'm developer.
I like to code.
i work at Torchwood."#;

    fn build_document() -> TextDocumentItemOwned {
        TextDocumentItemOwned::new(
            "uri://file".to_string(),
            "huml".to_string(),
            1,
            TEST_TEXT.to_string(),
        )
    }

    fn generate_update(substr: &str, replace_with: &str) -> (Range, String) {
        let test_text = TEST_TEXT.to_string();
        let substr_lines: Vec<_> = substr.lines().collect();

        let first_line = substr_lines
            .first()
            .expect("substr should have atleast one line");
        let last_line = substr_lines
            .last()
            .expect("substr should have atleast one line");

        let start = test_text
            .lines()
            .enumerate()
            .find_map(|(line, text)| {
                text.find(first_line)
                    .map(|character| Position::new(line, character))
            })
            .expect("Start position should be valid");

        let end = test_text
            .lines()
            .enumerate()
            .find_map(|(line, text)| {
                text.find(last_line)
                    .map(|character| Position::new(line, character + last_line.len()))
            })
            .expect("Start position should be valid");

        let updation_range = Range::new(start, end);
        let updated_string = test_text.trim().replace(substr, replace_with);

        (updation_range, updated_string)
    }

    fn generate_insertion_before(substr: &str, replace_with: &str) -> (Range, String) {
        let mut test_text = TEST_TEXT.to_string();
        let first_substr_line = substr
            .lines()
            .next()
            .expect("Atleast one line should be present");

        let found_at = test_text
            .find(substr)
            .expect("Substr should be part of the TEST_TEXT");

        let ins_position = test_text
            .lines()
            .enumerate()
            .find_map(|(line, text)| {
                text.find(first_substr_line)
                    .map(|character| Position::new(line, character))
            })
            .expect("Start position should be valid");

        let ins_range = Range::new(ins_position, ins_position);
        test_text.insert_str(found_at, replace_with);

        (ins_range, test_text)
    }

    fn generate_insertion_after(substr: &str, replace_with: &str) -> (Range, String) {
        let mut test_text = TEST_TEXT.to_string();
        let last_substr_line = substr
            .lines()
            .rev()
            .next()
            .expect("Atleast one line should be present");

        let found_at = test_text
            .find(substr)
            .expect("Substr should be part of the TEST_TEXT");

        let ins_position = test_text
            .lines()
            .enumerate()
            .find_map(|(line, text)| {
                text.find(last_substr_line)
                    .map(|character| Position::new(line, character + last_substr_line.len()))
            })
            .expect("Start position should be valid");

        let ins_range = Range::new(ins_position, ins_position);
        test_text.insert_str(found_at + substr.len(), replace_with);

        (ins_range, test_text)
    }

    fn generate_sentence_boundary_change(_: &str, replace_with: &str) -> (Range, String) {
        let text_clone = TEST_TEXT;
        let lines: Vec<_> = text_clone.lines().collect();
        let first = lines.first().unwrap();
        let boundary_range = Range::new(Position::new(0, first.len()), Position::new(1, 0));

        let last_few_chars = String::from(&first[first.len() - 4..]);
        let replace_pattern = format!("{last_few_chars}\n");
        let replace_with_pattern = format!("{last_few_chars}{replace_with}");
        (
            boundary_range,
            text_clone.replace(&replace_pattern, &replace_with_pattern),
        )
    }

    fn handle_test(
        substr: &str,
        replace_with: &str,
        generate_op: fn(&str, &str) -> (Range, String),
    ) -> (String, String) {
        let document = build_document();
        let line_seperated_document = LineSeperatedDocument::from(document);
        let (range, expected_text) = generate_op(substr, replace_with);
        let diff = [(range, replace_with)];
        let updated_text = line_seperated_document.apply_diff_to_document(&diff);
        (updated_text, expected_text)
    }

    fn handle_insert_before_test(substr: &str, replace_with: &str) -> (String, String) {
        handle_test(substr, replace_with, generate_insertion_before)
    }

    fn handle_insert_after_test(substr: &str, replace_with: &str) -> (String, String) {
        handle_test(substr, replace_with, generate_insertion_after)
    }

    fn handle_update_test(substr: &str, replace_with: &str) -> (String, String) {
        handle_test(substr, replace_with, generate_update)
    }

    fn handle_delete_test(substr: &str) -> (String, String) {
        handle_test(substr, "", generate_update)
    }

    #[test]
    fn should_update_single_word() {
        let (updated_text, expected_text) = handle_update_test("I'm", "Myself");
        assert_eq!(updated_text, expected_text);
    }

    #[test]
    fn should_update_first_word() {
        let (updated_text, expected_text) = handle_update_test("Hello", "Hi");
        assert_eq!(updated_text, expected_text);
    }

    #[test]
    fn should_update_last_word() {
        let (updated_text, expected_text) = handle_update_test("Torchwood.", "Regolith.");
        assert_eq!(updated_text, expected_text);
    }

    #[test]
    fn should_update_first_line() {
        let (updated_text, expected_text) =
            handle_update_test("Hello, I'm developer.", "Hi, I'm Joe.");
        assert_eq!(updated_text, expected_text);
    }

    #[test]
    fn should_update_middle_line() {
        let (updated_text, expected_text) = handle_update_test("I like to code.", "I like coding");
        assert_eq!(updated_text, expected_text);
    }

    #[test]
    fn should_update_last_line() {
        let (updated_text, expected_text) =
            handle_update_test("i work at Torchwood.", "I maintain HUML-LSP.");
        assert_eq!(updated_text, expected_text);
    }

    #[test]
    fn should_update_multiple_lines() {
        let (updated_text, expected_text) =
            handle_update_test("Hello, I'm developer.\nI like to code.", "Hello World");
        assert_eq!(updated_text, expected_text);
    }

    #[test]
    fn should_update_entire_text() {
        let (updated_text, expected_text) = handle_update_test(TEST_TEXT, "Hello World");
        assert_eq!(updated_text, expected_text);
    }
    #[test]
    fn should_insert_before_first_line() {
        let (after_insert, expected_text) = handle_insert_before_test("Hello", "Hello World");
        assert_eq!(after_insert, expected_text);
    }

    #[test]
    fn should_insert_after_last_char() {
        let (after_insert, expected_text) = handle_insert_after_test("code.", " when I'm bored.");
        assert_eq!(after_insert, expected_text);
    }

    #[test]
    fn should_update_sentence_boundary() {
        let (updated_text, expected_text) =
            handle_test(TEST_TEXT, "Hello World", generate_sentence_boundary_change);
        assert_eq!(updated_text, expected_text);
    }

    #[test]
    fn should_delete_sentence_boundary() {
        let (updated_text, expected_text) =
            handle_test(TEST_TEXT, "", generate_sentence_boundary_change);

        assert_eq!(updated_text, expected_text);
    }

    #[test]
    fn should_delete_first_word() {
        let (updated_text, expected_text) =
            handle_test(TEST_TEXT, "", generate_sentence_boundary_change);

        assert_eq!(updated_text, expected_text);
    }
}
