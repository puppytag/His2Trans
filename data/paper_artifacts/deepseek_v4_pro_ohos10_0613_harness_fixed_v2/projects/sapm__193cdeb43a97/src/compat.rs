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
// Source: /data/home/wangshb/c2-rust_framework/experiment_runs/deepseek-v4-pro-ohos10-full-0613-1/raw/framework_output/intermediate/sapm__193cdeb43a97/workspace/.preprocessed/*.i (preprocessed translation units)
// Details: /data/home/wangshb/c2-rust_framework/experiment_runs/deepseek-v4-pro-ohos10-full-0613-1/raw/framework_output/intermediate/sapm__193cdeb43a97/workspace/repair_history/sapm__193cdeb43a97/translate_by_qwen3_coder/_manual_fix/extern_decls_from_bindgen_allowlist.json
#[allow(improper_ctypes)]
#[allow(non_snake_case)]
extern "C" {
    pub fn AudioAddControl(audioCard: *const crate::types::AudioCard, ctl: *const crate::types::AudioKcontrol) -> *mut crate::types::AudioKcontrol;
    pub fn AudioCodecGetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCodecGetEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCodecMuxRegUpdate(codec: *mut crate::types::CodecDevice, enumCtrl: *mut crate::types::AudioEnumKcontrol, value: *const u32) -> i32;
    pub fn AudioCodecReadReg(codec: *const crate::types::CodecDevice, reg: u32, val: *mut u32) -> i32;
    pub fn AudioCodecRegUpdate(codec: *mut crate::types::CodecDevice, mixerCtrl: *mut crate::types::AudioMixerControl) -> i32;
    pub fn AudioKcontrolGetCodec(kcontrol: *const crate::types::AudioKcontrol) -> *mut crate::types::CodecDevice;
    pub fn AudioUpdateCodecRegBits(codec: *mut crate::types::CodecDevice, reg: u32, mask: u32, shift: u32, value: u32) -> i32;
    pub fn HiLogPrint(type_: crate::types::LogType, level: crate::types::LogLevel, domain: ::core::ffi::c_uint, tag: *const ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn OsalMemCalloc(size: crate::types::size_t) -> *mut ::core::ffi::c_void;
    pub fn OsalMemFree(mem: *mut ::core::ffi::c_void);
    pub fn OsalSleep(sec: u32);
    pub fn OsalThreadCreate(thread: *mut crate::types::OsalThread, threadEntry: crate::types::OsalThreadEntry, entryPara: *mut ::core::ffi::c_void) -> i32;
    pub fn OsalThreadDestroy(thread: *mut crate::types::OsalThread) -> i32;
    pub fn OsalThreadStart(thread: *mut crate::types::OsalThread, param: *const crate::types::OsalThreadParam) -> i32;
    pub fn memcpy_s(dest: *mut ::core::ffi::c_void, destMax: crate::types::size_t, src: *const ::core::ffi::c_void, count: crate::types::size_t) -> crate::types::errno_t;
    pub fn memset_s(dest: *mut ::core::ffi::c_void, destMax: crate::types::size_t, c: ::core::ffi::c_int, count: crate::types::size_t) -> crate::types::errno_t;
    pub fn snprintf_s(strDest: *mut ::core::ffi::c_char, destMax: crate::types::size_t, count: crate::types::size_t, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn strcmp(arg1: *const ::core::ffi::c_char, arg2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn strlen(arg1: *const ::core::ffi::c_char) -> crate::types::size_t;
}
// === C2R_EXTERN_DECLS_END ===

// === C2R_ACCESSOR_SHIMS_BEGIN ===
// (auto-appended accessor shim declarations will be inserted here)
// === C2R_ACCESSOR_SHIMS_END ===
