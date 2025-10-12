use serde::Deserialize;

use crate::lsp::{notification::ClientServerNotification, request::Request};

/// Any message recieved by the server:
/// Either a request or a notification
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RecievedMessage<'a> {
    #[serde(borrow)]
    Request(Request<'a>),
    Notification(ClientServerNotification<'a>),
}
