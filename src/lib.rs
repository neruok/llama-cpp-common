//! Bindings to llama.cpp's `common/` helpers.
//!
//! This crate is maintained separately from `llama-cpp-rs`, so the upstream
//! bindings crate does not carry the `common` surface.
//!
//! Status: skeleton. The Minja chat-template wrapper, the
//! JSON-schema-to-grammar helper, and the MTP speculative wrapper move here
//! from the `neruok/llama-cpp-rs` fork. See `docs/llama-cpp-common-crate.md`
//! in the workspace harness repository.

#![warn(missing_docs)]
