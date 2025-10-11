pub mod trace;
use serde::{Deserialize, Serialize};

use crate::lsp::notification::trace::SetTraceParams;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "camelCase")]
pub enum Notification {
    Initialized(InitializedParams),
    #[serde(rename = "$/setTrace")]
    SetTrace(SetTraceParams),
    Exit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializedParams {}
