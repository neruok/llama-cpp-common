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

The crate uses llama.cpp as a git submodule and builds the `common` subset it
needs with `cc`. It links `libllama` and the ggml libraries from
`llama-cpp-sys-2`. The submodule is pinned to the llama.cpp commit that
`llama-cpp-sys-2` pins. Run `git submodule update --init --depth 1` before a
build.

## Versioning

The build records the `llama.cpp` submodule commit in `LLAMA_CPP_COMMIT` and
`LLAMA_CPP_BUILD_NUMBER`. `llama-cpp-rs` exposes its pinned commit only as a
gitlink, so nothing checks the two submodule commits against each other. A
mismatch shows as a link error. Keep this submodule pinned to the commit that
`llama-cpp-sys-2` pins.

## Dependency on raw handles

The wrapper needs `llama_model *` and `llama_context *`. `llama-cpp-2` keeps
those `pub(crate)`. Public accessors are proposed upstream. Until they land, the
fork pin above carries them.

## License

MIT OR Apache-2.0, Copyright (c) 2026 neruok. See `THIRD-PARTY.md` for the
notices of the derived and vendored code.
