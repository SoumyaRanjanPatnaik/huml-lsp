# HUML Language Server

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](./LICENSE-MIT)

An implementation of the [Language Server Protocol (LSP)](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/) for the [HUML](https://huml.io) language, written in Rust.

## Overview

This project provides IDE support for the HUML (Human-oriented Markup Language), a simple and strict serialization language designed for human readability. By leveraging the Language Server Protocol, this server can provide features like diagnostics, code completion, and hover information to any editor that supports the protocol.

## Project Status

The project is currently being tested with neovim's LSP support. Currently, only the base protocol implementation is completed.
The next steps would be to implement the capabilities required to provide diagnostics and hover support.

## Server Setup

To setup the server, first clone the server to you desired location

```bash
git clone https://github.com/SoumyaRanjanPatnaik/huml-lsp.git

```

Then, run the following to build the project

```bash
cd huml-lsp
cargo build --release
```

### Connecting from Neovim (v0.11+)

To connect to `huml-lsp` from neovim v0.11+, add the following to `~/.config/nvim/lsp/huml_ls.lua`:

```lua
---@type vim.lsp.Config
return {
  name = "huml-lsp",
  cmd = { "/path/to/huml-lsp/target/release/huml-lsp" },
  filetypes = { "huml" },
}
```

Replace `/path/to/huml-lsp` with the location of the root of the cloned repository.

Then in `~/.config/nvim/init.lua`, add the following:

```lua
vim.lsp.enable("huml_ls")
```

### Connecting from VS Code

TODO: Develop a vscode extension to connect to the LSP server

## Milestones

- [ ] Support for Text Document Sync
- [ ] Diagnostics support
- [ ] Hover Support
