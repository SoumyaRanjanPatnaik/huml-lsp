use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// Define type aliases for all the base types
pub type Integer = i32;
pub type UInteger = usize;
pub type Decimal = u64;
pub type LSPArray = Vec<LSPAny>;
pub type LSPObject = HashMap<String, LSPAny>;

/// This enum represents any usable value in the JSON rpc specification
/// that is not null. This type is not in itself part of the spec,
/// but allows for marking types that would never be nullable.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LSPAny {
    LSPObject(LSPObject),
    LSPArray(LSPArray),
    String(String),
    Integer(Integer),
    UInteger(UInteger),
    Decimal(Decimal),
    Boolean(bool),
}
