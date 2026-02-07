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
// Source: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/claude-test0125/intermediate/urlparser/workspace/.preprocessed/*.i (preprocessed translation units)
// Details: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/claude-test0125/intermediate/urlparser/workspace/repair_history/urlparser/translate_by_qwen3_coder/_manual_fix/extern_decls_from_bindgen_allowlist.json
#[allow(improper_ctypes)]
#[allow(non_snake_case)]
extern "C" {
    pub fn free(__ptr: *mut ::core::ffi::c_void);
    pub fn malloc(__size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    pub fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn sprintf(__s: *mut ::core::ffi::c_char, __format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn sscanf(__s: *const ::core::ffi::c_char, __format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn strcat(__dest: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn strcpy(__dest: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn strlen(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_ulong;
    pub fn strstr(__haystack: *const ::core::ffi::c_char, __needle: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_get_auth(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_get_hash(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_get_host(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_get_hostname(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_get_path(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_get_pathname(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_get_port(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_get_protocol(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_get_query(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_get_search(url: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn url_parse(url: *mut ::core::ffi::c_char) -> *mut crate::types::url_data_t;
}
// === C2R_EXTERN_DECLS_END ===

// === C2R_ACCESSOR_SHIMS_BEGIN ===
// (auto-appended accessor shim declarations will be inserted here)
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__search(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__auth(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__host(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__hostname(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__href(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__protocol(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__pathname(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__hash(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__query(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__port(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__path(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
// === C2R_ACCESSOR_SHIMS_END ===
