//! Bindings to llama.cpp's `common/` helpers.
//!
//! This crate is maintained separately from `llama-cpp-rs`, so the upstream
//! bindings crate does not carry the `common` surface. It builds the `common`
//! sources from the `llama.cpp` submodule and links the `libllama` and ggml
//! libraries from `llama-cpp-sys-2`.
//!
//! It provides the Minja chat-template wrapper, the JSON-schema-to-grammar
//! helper, and the MTP speculative wrapper.

#![warn(missing_docs)]

pub mod chat;
pub mod grammar;
pub mod speculative;

mod sys;

pub use grammar::json_schema_to_grammar;
