mod initialize;
pub use initialize::*;

use crate::rpc::Integer;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Request {
    id: Integer,
    #[serde(flatten)]
    method: RequestMethods,
}

impl Request {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn method(&self) -> &RequestMethods {
        &self.method
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "method", content = "params")]
pub enum RequestMethods {
    Initialize(InitializeParams),
    Shutdown,
}
