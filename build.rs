//! Build the llama.cpp `common` sources from the `llama.cpp` submodule and the
//! `llama_rs_*` wrapper.
//!
//! The wrapper links against the `libllama` and ggml libraries that
//! `llama-cpp-sys-2` builds. The submodule is pinned to the llama.cpp revision
//! that `llama-cpp-sys-2` pins.
//!
//! The build records the submodule commit in `LLAMA_CPP_COMMIT` and
//! `LLAMA_CPP_BUILD_NUMBER`, and generates the `build-info.cpp` that
//! `common.cpp` calls.

use std::path::{Path, PathBuf};
use std::process::Command;

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

    if !llama.join("common").is_dir() {
        panic!(
            "the llama.cpp submodule is not initialized at {}\nrun: git submodule update --init --depth 1",
            llama.display()
        );
    }

    for file in ["wrapper_common.cpp", "wrapper_common.h", "wrapper_utils.h", ".gitmodules"] {
        println!("cargo:rerun-if-changed={file}");
    }
    for dir in ["common", "include", "ggml/include", "src", "vendor"] {
        println!("cargo:rerun-if-changed={}", llama.join(dir).display());
    }

    // The commit is the version identity. `rev-list --count` is 1 for a shallow
    // submodule, which is what this workspace uses.
    let commit = git(&llama, &["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let build_number = git(&llama, &["rev-list", "--count", "HEAD"]).unwrap_or_else(|| "0".to_string());
    println!("cargo:rustc-env=LLAMA_CPP_COMMIT={commit}");
    println!("cargo:rustc-env=LLAMA_CPP_BUILD_NUMBER={build_number}");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR is set"));
    let build_info = out_dir.join("build-info.cpp");
    std::fs::write(&build_info, build_info_source(&commit, &build_number))
        .unwrap_or_else(|err| panic!("cannot write {}: {err}", build_info.display()));

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
        .file(&build_info);

    add_cpp_dir(&mut build, &llama.join("common"), EXCLUDED);
    add_cpp_dir(&mut build, &llama.join("common").join("jinja"), &[]);

    build.compile("llama_cpp_common");
}

/// The `build-info.cpp` that `common.cpp` expects, with the submodule commit.
fn build_info_source(commit: &str, build_number: &str) -> String {
    format!(
        r#"#include "build-info.h"

#include <cstdio>
#include <string>

int LLAMA_BUILD_NUMBER = {build_number};
char const * LLAMA_COMMIT = "{commit}";
char const * LLAMA_COMPILER = "unknown";
char const * LLAMA_BUILD_TARGET = "unknown";

int llama_build_number(void) {{
    return LLAMA_BUILD_NUMBER;
}}

const char * llama_commit(void) {{
    return LLAMA_COMMIT;
}}

const char * llama_compiler(void) {{
    return LLAMA_COMPILER;
}}

const char * llama_build_target(void) {{
    return LLAMA_BUILD_TARGET;
}}

const char * llama_build_info(void) {{
    static std::string s = "b" + std::to_string(LLAMA_BUILD_NUMBER) + "-" + LLAMA_COMMIT;
    return s.c_str();
}}

void llama_print_build_info(const char * llama_version) {{
    fprintf(stderr, "version: %s (build %d, commit %s)\n", llama_version, llama_build_number(), llama_commit());
}}
"#
    )
}

fn git(dir: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8(output.stdout).ok()?;
    let text = text.trim().to_owned();
    if text.is_empty() { None } else { Some(text) }
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
