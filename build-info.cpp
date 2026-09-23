// Build information for the llama.cpp `common` sources.
//
// The CMake build generates this file from `build-info.cpp.in`. This crate does
// not run CMake for `common`, so the values are fixed here. `common.cpp` calls
// these functions.

#include "build-info.h"

#include <cstdio>
#include <string>

int LLAMA_BUILD_NUMBER = 0;
char const * LLAMA_COMMIT = "vendored";
char const * LLAMA_COMPILER = "unknown";
char const * LLAMA_BUILD_TARGET = "unknown";

int llama_build_number(void) {
    return LLAMA_BUILD_NUMBER;
}

const char * llama_commit(void) {
    return LLAMA_COMMIT;
}

const char * llama_compiler(void) {
    return LLAMA_COMPILER;
}

const char * llama_build_target(void) {
    return LLAMA_BUILD_TARGET;
}

const char * llama_build_info(void) {
    static std::string s = "vendored";
    return s.c_str();
}

void llama_print_build_info(const char * llama_version) {
    fprintf(stderr, "version: %s (vendored)\n", llama_version);
}
