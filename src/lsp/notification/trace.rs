use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SetTraceParams {
    value: TraceValue,
}

impl SetTraceParams {
    pub fn value(&self) -> &TraceValue {
        &self.value
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum TraceValue {
    Off,
    Message,
    Verbose,
}
