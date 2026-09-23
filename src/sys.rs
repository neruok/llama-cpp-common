//! Hand-written FFI declarations for `wrapper_common.cpp`.
//!
//! `llama-cpp-sys-2` exposes the `llama_*` C API. It exposes the `llama_rs_*`
//! wrappers only under its own `common` feature, which this crate replaces. The
//! declarations here bind the same symbols.

#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int, c_void};

use llama_cpp_sys_2::{
    llama_batch, llama_chat_message, llama_context, llama_model, llama_pos, llama_token,
};

/// `llama_rs_status` from `wrapper_utils.h`.
pub type llama_rs_status = c_int;

/// The success value of [`llama_rs_status`].
pub const LLAMA_RS_STATUS_OK: llama_rs_status = 0;

/// The draft output exceeded the caller-provided capacity.
pub const LLAMA_RS_STATUS_ALLOCATION_FAILED: llama_rs_status = -2;

extern "C" {
    pub fn llama_rs_json_schema_to_grammar(
        schema_json: *const c_char,
        force_gbnf: bool,
        out_grammar: *mut *mut c_char,
    ) -> llama_rs_status;

    pub fn llama_rs_string_free(ptr: *mut c_char);

    pub fn llama_rs_chat_template_init(
        model: *const llama_model,
        tmpl: *const c_char,
    ) -> *mut c_void;

    pub fn llama_rs_chat_template_free(tmpls: *mut c_void);

    pub fn llama_rs_chat_template_apply(
        tmpls: *const c_void,
        messages: *const llama_chat_message,
        n_messages: usize,
        add_generation_prompt: bool,
        enable_thinking: bool,
        kwarg_keys: *const *const c_char,
        kwarg_values: *const *const c_char,
        n_kwargs: usize,
        out_prompt: *mut *mut c_char,
    ) -> llama_rs_status;

    pub fn llama_rs_mtp_speculative_init(
        ctx_tgt: *mut llama_context,
        ctx_dft: *mut llama_context,
        n_max: i32,
        n_min: i32,
        p_min: f32,
    ) -> *mut c_void;

    pub fn llama_rs_mtp_speculative_free(spec: *mut c_void);

    pub fn llama_rs_mtp_speculative_begin(
        spec: *mut c_void,
        prompt_tokens: *const llama_token,
        prompt_tokens_count: usize,
    ) -> llama_rs_status;

    pub fn llama_rs_mtp_speculative_process(
        spec: *mut c_void,
        batch: *const llama_batch,
    ) -> llama_rs_status;

    pub fn llama_rs_mtp_speculative_draft(
        spec: *mut c_void,
        n_past: llama_pos,
        id_last: llama_token,
        prompt_tokens: *const llama_token,
        prompt_tokens_count: usize,
        out_tokens: *mut llama_token,
        out_tokens_capacity: usize,
        out_tokens_count: *mut usize,
    ) -> llama_rs_status;

    pub fn llama_rs_mtp_speculative_accept(
        spec: *mut c_void,
        n_accepted: u16,
    ) -> llama_rs_status;
}
