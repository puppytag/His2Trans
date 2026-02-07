//! Compatibility / Fallback Layer
//!
//! This module is auto-generated to keep the translated project compiling.
//!
//! Design goals:
//! - Centralize placeholders and shims in ONE place (easy to audit & remove later).
//! - Keep function bodies as close to translated semantics as possible.
//!
//! IMPORTANT:
//! - Items here may be placeholders (value/layout unknown). Always review before relying on semantics.

#![allow(dead_code)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

/// Minimal FFI prelude (for legacy skeletons that import `crate::compat::ffi::*`).
pub mod ffi {
    pub use core::ffi::*;
}

// === C2R_C2RUST_FALLBACK_MODS_BEGIN ===
// Auto-generated: C2Rust transpile fallback modules (used when LLM translation fails).
// NOTE: This is NOT truth-layer; it is a deterministic mechanical fallback.
#[path = "__c2r_generated/c2rust_fallback/mod.rs"]
pub mod __c2rust_fallback;
// === C2R_C2RUST_FALLBACK_MODS_END ===

// === C2R_COMPAT_PLACEHOLDERS_BEGIN ===
// (auto-appended placeholders will be inserted here)
// === C2R_COMPAT_PLACEHOLDERS_END ===

// === C2R_EXTERN_DECLS_BEGIN ===
// Auto-generated extern decls (C2R step 2.55; bindgen allowlist).
// Source: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/rgba/workspace/.preprocessed/*.i (preprocessed translation units)
// Details: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/rgba/workspace/repair_history/rgba/translate_by_qwen3_coder/_manual_fix/extern_decls_from_bindgen_allowlist.json
#[allow(improper_ctypes)]
#[allow(non_snake_case)]
extern "C" {
    pub fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn snprintf(__s: *mut ::core::ffi::c_char, __maxlen: ::core::ffi::c_ulong, __format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn strlen(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_ulong;
    pub fn strstr(__haystack: *const ::core::ffi::c_char, __needle: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
// === C2R_EXTERN_DECLS_END ===

// === C2R_ACCESSOR_SHIMS_BEGIN ===
// (auto-appended accessor shim declarations will be inserted here)
// === C2R_ACCESSOR_SHIMS_END ===
