//! Build the llama.cpp `common` sources from the `llama.cpp` submodule and the
//! `llama_rs_*` wrapper.
//!
//! The wrapper links against the `libllama` and ggml libraries that
//! `llama-cpp-sys-2` builds. The submodule is pinned to the llama.cpp revision
//! that `llama-cpp-sys-2` pins.

use std::path::{Path, PathBuf};

/// `common` sources that need cpp-httplib, curl, subprocess, or llguidance.
/// This crate does not use those helpers.
const EXCLUDED: &[&str] = &[
    "arg.cpp",
    "console.cpp",
    "download.cpp",
    "hf-cache.cpp",
    "imatrix-loader.cpp",
    "llguidance.cpp",
    "preset.cpp",
    "subproc.cpp",
];

fn main() {
    let manifest =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set"));
    let llama = manifest.join("llama.cpp");

    for file in [
        "wrapper_common.cpp",
        "wrapper_common.h",
        "wrapper_utils.h",
        "build-info.cpp",
    ] {
        println!("cargo:rerun-if-changed={file}");
    }
    println!("cargo:rerun-if-changed={}", llama.display());

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .pic(true)
        .warnings(false)
        .include(&llama)
        .include(llama.join("common"))
        .include(llama.join("include"))
        .include(llama.join("ggml/include"))
        .include(llama.join("vendor"))
        .file(manifest.join("wrapper_common.cpp"))
        .file(manifest.join("build-info.cpp"));

    add_cpp_dir(&mut build, &llama.join("common"), EXCLUDED);
    add_cpp_dir(&mut build, &llama.join("common").join("jinja"), &[]);

    build.compile("llama_cpp_common");
}

fn add_cpp_dir(build: &mut cc::Build, dir: &Path, excluded: &[&str]) {
    let entries =
        std::fs::read_dir(dir).unwrap_or_else(|err| panic!("cannot read {}: {err}", dir.display()));
    let mut files: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("cpp"))
        .collect();
    files.sort();
    for path in files {
        let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("");
        if !excluded.contains(&name) {
            build.file(&path);
        }
    }
}
