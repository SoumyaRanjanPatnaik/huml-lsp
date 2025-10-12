//! The `lsp` module is the core of the language server implementation.
//!
//! It is responsible for handling the communication and data structures defined
//! by the Language Server Protocol. This module orchestrates the server's lifecycle,
//! message dispatching, and defines the specific types for requests, responses,
//! notifications, and errors that are exchanged between the server and the client (the editor or IDE).
//!
//! The module is broken down into several submodules, each with a distinct responsibility
//! in the protocol's implementation.

/// Defines the error types and codes used in LSP responses.
pub mod error;

/// Contains the definitions for all LSP notification messages.
pub mod notification;

/// Defines common data structures and types used throughout the LSP.
pub mod properties;

/// Defines types related to LSP capabilities
pub mod capabilities;

/// Provides a structure for deserializing any incoming message from the client.
pub mod recieved_message;

/// Contains the definitions for all LSP request messages.
pub mod request;

/// Contains the definitions for all LSP response messages.
pub mod response;

/// Contains the definitions of  common JSON structures used in the LSP specification
pub mod common;

/// Contains the server state and request handlers
pub mod server;
