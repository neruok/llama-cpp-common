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

/// The pinned llama.cpp commit (short form), recorded at build time from the
/// `llama.cpp` submodule.
pub const LLAMA_CPP_COMMIT: &str = env!("LLAMA_CPP_COMMIT");

/// The llama.cpp commit count, recorded at build time. A shallow submodule
/// reports `1`.
pub const LLAMA_CPP_BUILD_NUMBER: &str = env!("LLAMA_CPP_BUILD_NUMBER");
