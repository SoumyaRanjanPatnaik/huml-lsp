use std::{num::ParseIntError, str::Utf8Error};

#[derive(thiserror::Error, Debug)]
pub enum CodingError {
    #[error("Failed to encode data: {0}")]
    EncodeFailed(#[from] EncodeError),
    #[error("Failed to decode data: {0}")]
    DecodeFailed(#[from] DecodeError),
}

#[derive(thiserror::Error, Debug)]
#[error("Encode failed due to JSON error: {0}")]
pub struct EncodeError(#[from] serde_json::Error);

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Missing or invalid header in the data.")]
    MissingOrInvalidHeader,
    #[error("Error converting content length to utf8. {0}")]
    InvalidContentLengthEncoding(Utf8Error),
    #[error("Error converting content length to usize. {0}")]
    ContentLengthNotNumber(ParseIntError),
    #[error("Data length does not match Content-Length")]
    IncompleteData,
    #[error("JSON deserialization error: {0}")]
    JsonError(#[from] serde_json::Error),
}
