pub mod trace;
use serde::{Deserialize, Serialize};

use crate::lsp::notification::trace::{LogTraceParams, SetTraceParams};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "camelCase")]
pub enum ClientServerNotification {
    Initialized(InitializedParams),
    #[serde(rename = "$/setTrace")]
    SetTrace(SetTraceParams),
    Exit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializedParams {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method", content = "params")]
#[serde(rename_all = "camelCase")]
pub enum ServerClientNotification {
    LogTrace(LogTraceParams),
}

impl From<LogTraceParams> for ServerClientNotification {
    fn from(v: LogTraceParams) -> Self {
        Self::LogTrace(v)
    }
}
