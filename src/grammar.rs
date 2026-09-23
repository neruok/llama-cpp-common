//! JSON-schema-to-grammar conversion (PH-4a, RD-19).
//!
//! Wraps llama.cpp's `common` `json_schema_to_grammar`. The grammar string
//! constrains decoding through a `LlamaSampler::grammar`.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::sys;

/// Errors from [`json_schema_to_grammar`].
#[derive(Debug, thiserror::Error)]
pub enum GrammarError {
    /// The schema contains an interior null byte.
    #[error("the schema contains a null byte")]
    NulByte(#[from] std::ffi::NulError),
    /// llama.cpp rejected the schema.
    #[error("llama.cpp failed to convert the schema with status {0}")]
    Ffi(i32),
    /// The returned grammar was not valid utf8.
    #[error("the grammar was not valid utf8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

/// Convert a JSON schema string into a llama.cpp grammar string.
///
/// # Errors
///
/// Returns [`GrammarError`] when the schema contains a null byte, llama.cpp
/// rejects it, or the result is not valid utf8.
pub fn json_schema_to_grammar(schema_json: &str) -> Result<String, GrammarError> {
    let schema = CString::new(schema_json)?;
    let mut out: *mut c_char = std::ptr::null_mut();
    let status = unsafe { sys::llama_rs_json_schema_to_grammar(schema.as_ptr(), false, &mut out) };
    if status != sys::LLAMA_RS_STATUS_OK || out.is_null() {
        return Err(GrammarError::Ffi(status));
    }
    let grammar = unsafe { CStr::from_ptr(out) }.to_str()?.to_owned();
    unsafe { sys::llama_rs_string_free(out) };
    Ok(grammar)
}
