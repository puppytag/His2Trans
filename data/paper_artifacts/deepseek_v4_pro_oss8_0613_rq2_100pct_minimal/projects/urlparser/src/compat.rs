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


// === C2R_ACCESSOR_SHIMS_BEGIN ===
// (auto-appended accessor shim declarations will be inserted here)
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__protocol(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__hostname(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__host(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__pathname(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__href(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__path(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
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
    pub fn c2r_field_ptr_url_data_t__auth(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
#[allow(improper_ctypes)]
extern "C" {
    pub fn c2r_field_ptr_url_data_t__search(base: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
// === C2R_ACCESSOR_SHIMS_END ===