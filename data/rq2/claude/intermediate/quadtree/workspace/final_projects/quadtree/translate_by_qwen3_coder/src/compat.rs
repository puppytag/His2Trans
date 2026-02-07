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
// Source: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/claude-test0125/intermediate/quadtree/workspace/.preprocessed/*.i (preprocessed translation units)
// Details: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/claude-test0125/intermediate/quadtree/workspace/repair_history/quadtree/translate_by_qwen3_coder/_manual_fix/extern_decls_from_bindgen_allowlist.json
#[allow(improper_ctypes)]
#[allow(non_snake_case)]
extern "C" {
    pub fn fabs(__x: f64) -> f64;
    pub fn fmax(__x: f64, __y: f64) -> f64;
    pub fn fmin(__x: f64, __y: f64) -> f64;
    pub fn free(__ptr: *mut ::core::ffi::c_void);
    pub fn malloc(__size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
}
// === C2R_EXTERN_DECLS_END ===

// === C2R_ACCESSOR_SHIMS_BEGIN ===
// (auto-appended accessor shim declarations will be inserted here)
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_point_t__x(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_point_t__y(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_node_t__point(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_bounds_t__se(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_bounds_t__width(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_bounds_t__height(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_bounds_t__nw(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_node_t__bounds(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_node_t__nw(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_node_t__sw(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_node_t__se(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_node_t__ne(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_node_t__key(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_t__key_free(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_t__root(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_quadtree_t__length(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
// === C2R_ACCESSOR_SHIMS_END ===
