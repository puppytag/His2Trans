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
// Source: /data/home/wangshb/c2-rust_framework/experiment_runs/deepseek-v4-pro-ohos10-full-0613-1/raw/framework_output/intermediate/shared__541f4e547bdb/workspace/.preprocessed/*.i (preprocessed translation units)
// Details: /data/home/wangshb/c2-rust_framework/experiment_runs/deepseek-v4-pro-ohos10-full-0613-1/raw/framework_output/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/extern_decls_from_bindgen_allowlist.json
#[allow(improper_ctypes)]
#[allow(non_snake_case)]
extern "C" {
    pub fn HdfDeviceRegisterEventListener(target: *mut crate::types::HdfIoService, listener: *mut crate::types::HdfDevEventlistener) -> ::core::ffi::c_int;
    pub fn HdfDeviceUnregisterEventListener(target: *mut crate::types::HdfIoService, listener: *mut crate::types::HdfDevEventlistener) -> ::core::ffi::c_int;
    pub fn HdfIoServiceAdapterObtain(serviceName: *const ::core::ffi::c_char) -> *mut crate::types::HdfIoService;
    pub fn HdfIoServiceAdapterRecycle(service: *mut crate::types::HdfIoService);
    pub static mut HdfIoServiceAdapterPublish: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, u32) -> *mut crate::types::HdfIoService>;
    pub static mut HdfIoServiceAdapterRemove: Option<unsafe extern "C" fn(*mut crate::types::HdfIoService)>;
    pub fn HdfIoserviceGetListenerCount(service: *const crate::types::HdfIoService) -> ::core::ffi::c_int;
    pub fn HdfObjectManagerGetCreators(objectId: ::core::ffi::c_int) -> *const crate::types::HdfObjectCreator;
    pub fn HdfSbufObtainDefaultSize() -> *mut crate::types::HdfSBuf;
    pub fn HdfSbufReadString(sbuf: *mut crate::types::HdfSBuf) -> *const ::core::ffi::c_char;
    pub fn HdfSbufReadUint16(sbuf: *mut crate::types::HdfSBuf, value: *mut u16) -> bool;
    pub fn HdfSbufRecycle(sbuf: *mut crate::types::HdfSBuf);
    pub fn HdfSbufWriteString(sbuf: *mut crate::types::HdfSBuf, value: *const ::core::ffi::c_char) -> bool;
    pub fn HdfSbufWriteUint16(sbuf: *mut crate::types::HdfSBuf, value: u16) -> bool;
    pub fn HiLogPrint(type_: crate::types::LogType, level: crate::types::LogLevel, domain: ::core::ffi::c_uint, tag: *const ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn OsalMemCalloc(size: crate::types::size_t) -> *mut ::core::ffi::c_void;
    pub fn OsalMemFree(mem: *mut ::core::ffi::c_void);
}
// === C2R_EXTERN_DECLS_END ===

// === C2R_ACCESSOR_SHIMS_BEGIN ===
// (auto-appended accessor shim declarations will be inserted here)
// === C2R_ACCESSOR_SHIMS_END ===
