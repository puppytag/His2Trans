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

// === C2R_COMPAT_PLACEHOLDERS_BEGIN ===
// (auto-appended placeholders will be inserted here)
// === C2R_COMPAT_PLACEHOLDERS_END ===

// === C2R_EXTERN_DECLS_BEGIN ===
// Auto-generated extern decls (C2R step 2.55; bindgen allowlist).
// Source: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/claude-test0125/intermediate/genann/workspace/.preprocessed/*.i (preprocessed translation units)
// Details: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/claude-test0125/intermediate/genann/workspace/repair_history/genann/translate_by_qwen3_coder/_manual_fix/extern_decls_from_bindgen_allowlist.json
#[allow(improper_ctypes)]
#[allow(non_snake_case)]
extern "C" {
    pub fn __assert_fail(__assertion: *const ::core::ffi::c_char, __file: *const ::core::ffi::c_char, __line: ::core::ffi::c_uint, __function: *const ::core::ffi::c_char) -> !;
    pub fn __errno_location() -> *mut ::core::ffi::c_int;
    pub fn exp(__x: f64) -> f64;
    pub fn fprintf(__stream: *mut crate::types::FILE, __format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn free(__ptr: *mut ::core::ffi::c_void);
    pub fn fscanf(__stream: *mut crate::types::FILE, __format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn malloc(__size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    pub fn memcpy(__dest: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    pub fn perror(__s: *const ::core::ffi::c_char);
    pub fn rand() -> ::core::ffi::c_int;
}
// === C2R_EXTERN_DECLS_END ===

// === C2R_ACCESSOR_SHIMS_BEGIN ===
// (auto-appended accessor shim declarations will be inserted here)
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_genann__weight(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_genann__total_weights(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
// === C2R_ACCESSOR_SHIMS_END ===
