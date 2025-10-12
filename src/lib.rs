//! This crate provides the implementation of the language server protocol
//! for [HUML](https://huml.io).
//!
//! # Overview
//!
//! This library allows editors and IDEs that support the [Language Server Protocol (LSP)](https://microsoft.github.io/language-server-protocol/)
//! to provide features like code completion, diagnostics, and hover information for the
//! [HUML](https://huml.io) language. HUML, or Human-oriented Markup Language, is a simple and strict
//! serialization language designed for human readability, often used for documents, datasets, and configurations.
//!
//! The crate is structured into three main modules: `huml`, `rpc`, and `lsp`.
//!
//! ## Modules
//!
//! - **`rpc`**: This module handles the JSON-RPC communication between the language server and the client (the editor or IDE). It is responsible for serializing and deserializing the LSP messages that are exchanged.
//!
//! - **`lsp`**: This is the core module that implements the `LanguageServer` trait. It connects the `huml` parser with the `rpc` communication layer. It receives notifications and requests from the client, such as `textDocument/didOpen`, `textDocument/hover`, or `textDocument/completion`, and uses the `huml` module to provide the appropriate responses.

pub mod lsp;
pub mod rpc;
