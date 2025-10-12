//! The `rpc` module is responsible for handling the JSON-RPC 2.0 transport layer.
//!
//! The Language Server Protocol is built on top of JSON-RPC, which defines the
//! structure of the messages exchanged between the client (editor/IDE) and the
//! server. This module provides the necessary components to encode, decode,
//! and transport these messages, abstracting the raw communication from the
//! main language server logic in the `lsp` module.
//!
//! It manages message framing (e.g., `Content-Length` headers), serialization
//! and deserialization between JSON and Rust structs, and reading from/writing to
//! the underlying I/O streams (typically `stdin` and `stdout`).

/// Handles the encoding and decoding of JSON-RPC messages.
mod coding;

/// Defines errors specific to the JSON-RPC communication layer.
mod error;

/// Manages the transport layer for sending and receiving messages.
mod transport;

/// Defines the core data structures of a JSON-RPC message.
mod types;

// Re-export the public items from the submodules for easier access.
pub use coding::*;
pub use error::*;
pub use transport::*;
pub use types::*;
