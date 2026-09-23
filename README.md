# llama-cpp-common

Bindings to llama.cpp's `common/` helpers, maintained separately from
[`llama-cpp-rs`](https://github.com/utilityai/llama-cpp-rs). The upstream
bindings crate does not carry this surface.

Status: skeleton. The wrapper is not implemented yet.

## What moves here

From the `neruok/llama-cpp-rs` fork, behind its `common` feature:

- The Minja chat-template wrapper (`llama-cpp-2/src/chat.rs`).
- `json_schema_to_grammar` (`llama-cpp-2/src/lib.rs`).
- MTP speculative decoding (`llama-cpp-2/src/speculative.rs`).

## How it builds

The crate builds the llama.cpp `common` subset it needs with `cc`. It uses the
llama.cpp source tree that `llama-cpp-sys-2` exports
(`DEP_LLAMA_LLAMA_CPP_SOURCE`), so it needs no submodule of its own. It links
the `libllama` and ggml libraries from the same `llama-cpp-sys-2` build, so the
`common` sources and `libllama` always come from one revision. A local
`llama.cpp` directory is a development fallback.

## Versioning

The build records the llama.cpp revision in `LLAMA_CPP_COMMIT` and
`LLAMA_CPP_BUILD_NUMBER`, taken from `DEP_LLAMA_LLAMA_CPP_REV`. Because the
source tree and `libllama` come from the same `llama-cpp-sys-2` build, there is
no separate submodule to drift.

## Dependency on raw handles

The wrapper needs `llama_model *` and `llama_context *`. `llama-cpp-2` keeps
those `pub(crate)`. Public accessors are proposed upstream. Until they land, the
fork pin above carries them.

## License

MIT OR Apache-2.0, Copyright (c) 2026 neruok. See `THIRD-PARTY.md` for the
notices of the derived and vendored code.
