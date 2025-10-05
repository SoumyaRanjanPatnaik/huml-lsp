use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method", content = "params")]
pub enum Notification {
    Initialized(InitializedParams),
    Exit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializedParams {}
