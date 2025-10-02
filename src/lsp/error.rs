use thiserror::Error;

#[derive(Error, Debug)]
#[error("An unknown error occurred when handling the error")]
pub struct ServerError;
