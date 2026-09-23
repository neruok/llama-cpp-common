# Third-party notices

`llama-cpp-common` is licensed MIT OR Apache-2.0. It contains code and source
derived from other projects. Their notices apply to those portions.

## Portions derived from `llama-cpp-rs`

The chat-template wrapper, the JSON-schema-to-grammar helper, and the MTP
speculative wrapper are ported from `llama-cpp-2`.

- Project: `llama-cpp-rs` (https://github.com/utilityai/llama-cpp-rs)
- License: MIT OR Apache-2.0
- MIT copyright: Copyright (c) Dial AI

## llama.cpp submodule

`llama.cpp/` is a git submodule pinned to the commit that `llama-cpp-sys-2`
pins. Its `common` sources are built against the `llama-cpp-sys-2` `libllama`
and ggml libraries.

- Project: `llama.cpp` (https://github.com/ggml-org/llama.cpp)
- License: MIT
- Copyright: Copyright (c) 2023-2026 The ggml authors

The full llama.cpp license text is in the submodule at `llama.cpp/LICENSE`.
