//! Build the llama.cpp `common` sources from the tree that `llama-cpp-sys-2`
//! exports, and the `llama_rs_*` wrapper.
//!
//! `llama-cpp-sys-2` exports its pinned llama.cpp source tree as
//! `DEP_LLAMA_LLAMA_CPP_SOURCE` and `DEP_LLAMA_LLAMA_CPP_REV`. This crate builds
//! `common` from that same tree, so it needs no submodule of its own. A local
//! `llama.cpp` directory is a development fallback.
//!
//! The build records the revision in `LLAMA_CPP_COMMIT` and
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

    // Prefer the tree that llama-cpp-sys-2 built libllama from, so the two
    // always match. Fall back to a local submodule for standalone development.
    let llama = std::env::var_os("DEP_LLAMA_LLAMA_CPP_SOURCE")
        .map(PathBuf::from)
        .filter(|path| path.join("common").is_dir())
        .unwrap_or_else(|| manifest.join("llama.cpp"));

    if !llama.join("common").is_dir() {
        panic!(
            "no llama.cpp source tree at {}\n\
             llama-cpp-sys-2 exports DEP_LLAMA_LLAMA_CPP_SOURCE; check that the pinned \
             llama-cpp-sys-2 revision provides it, or add a llama.cpp submodule",
            llama.display()
        );
    }

    for file in ["wrapper_common.cpp", "wrapper_common.h", "wrapper_utils.h"] {
        println!("cargo:rerun-if-changed={file}");
    }
    for dir in ["common", "include", "ggml/include", "src", "vendor"] {
        println!("cargo:rerun-if-changed={}", llama.join(dir).display());
    }
    println!("cargo:rerun-if-env-changed=DEP_LLAMA_LLAMA_CPP_SOURCE");
    println!("cargo:rerun-if-env-changed=DEP_LLAMA_LLAMA_CPP_REV");

    // The exported revision identifies the tree. `rev-list --count` needs git
    // metadata, which a Cargo checkout may not carry, so it may report 0.
    let commit = std::env::var("DEP_LLAMA_LLAMA_CPP_REV")
        .ok()
        .filter(|rev| !rev.is_empty() && rev != "unknown")
        .or_else(|| git(&llama, &["rev-parse", "HEAD"]))
        .unwrap_or_else(|| "unknown".to_owned());
    let build_number =
        git(&llama, &["rev-list", "--count", "HEAD"]).unwrap_or_else(|| "0".to_owned());
    println!("cargo:rustc-env=LLAMA_CPP_COMMIT={commit}");
    println!("cargo:rustc-env=LLAMA_CPP_BUILD_NUMBER={build_number}");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR is set"));
    let build_info = out_dir.join("build-info.cpp");
    std::fs::write(&build_info, build_info_source(&commit, &build_number))
        .unwrap_or_else(|err| panic!("cannot write {}: {err}", build_info.display()));

    // The wrapper includes paths of the form `llama.cpp/...`, so the parent of
    // the source tree is on the include path.
    let parent = llama
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| manifest.clone());

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .pic(true)
        .warnings(false)
        .include(&parent)
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

/// The `build-info.cpp` that `common.cpp` expects, with the llama.cpp revision.
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
