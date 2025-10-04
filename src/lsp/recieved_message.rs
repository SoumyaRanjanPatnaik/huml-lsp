use serde::Deserialize;

use crate::lsp::{notification::Notification, request::Request};

/// Any message recieved by the server:
/// Either a request or a notification
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RecievedMessage {
    Request(Request),
    Notification(Notification),
}
