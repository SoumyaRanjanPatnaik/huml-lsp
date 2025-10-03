use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Failed to initialize server")]
    Initialize(#[from] InitializeError),
}

#[derive(Error, Debug)]
pub enum InitializeError {
    #[error("Server already initialized")]
    AlreadyInitialized,
}
