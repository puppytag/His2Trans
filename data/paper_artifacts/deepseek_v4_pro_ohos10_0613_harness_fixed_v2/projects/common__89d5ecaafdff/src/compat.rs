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
// Source: archived_runs/deepseek-v4-pro-ohos10-full-0613-1/raw/framework_output/intermediate/common__89d5ecaafdff/workspace/.preprocessed/*.i (preprocessed translation units)
// Details: archived_runs/deepseek-v4-pro-ohos10-full-0613-1/raw/framework_output/intermediate/common__89d5ecaafdff/workspace/repair_history/common__89d5ecaafdff/translate_by_qwen3_coder/_manual_fix/extern_decls_from_bindgen_allowlist.json
#[allow(improper_ctypes)]
#[allow(non_snake_case)]
extern "C" {
    pub fn AudioDaiRegUpdate(dai: *const crate::types::DaiDevice, mixerCtrl: *mut crate::types::AudioMixerControl) -> i32;
    pub fn AudioGetPortConfig(device: *const crate::types::HdfDeviceObject, configData: *mut crate::types::AudioPortInfo) -> i32;
    pub fn AudioGetRegConfig(device: *const crate::types::HdfDeviceObject, configData: *mut crate::types::AudioRegCfgData) -> i32;
    pub fn AudioSampPowerUp(card: *const crate::types::AudioCard) -> i32;
    pub fn AudioSampSetPowerMonitor(card: *mut crate::types::AudioCard, powerMonitorState: bool) -> i32;
    pub fn DeviceResourceGetIfaceInstance(type_: crate::types::DeviceResourceType) -> *mut crate::types::DeviceResourceIface;
    pub fn HdfDeviceObjectSetServInfo(dev: *mut crate::types::HdfDeviceObject, info: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn HdfDeviceObjectUpdate(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int;
    pub fn HiLogPrint(type_: crate::types::LogType, level: crate::types::LogLevel, domain: ::core::ffi::c_uint, tag: *const ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn I2cClose(handle: crate::types::DevHandle);
    pub fn I2cOpen(number: i16) -> crate::types::DevHandle;
    pub fn I2cTransfer(handle: crate::types::DevHandle, msgs: *mut crate::types::I2cMsg, count: i16) -> i32;
    pub fn OsalMSleep(ms: u32);
    pub fn OsalMemCalloc(size: crate::types::size_t) -> *mut ::core::ffi::c_void;
    pub fn OsalMemFree(mem: *mut ::core::ffi::c_void);
    pub fn memcpy_s(dest: *mut ::core::ffi::c_void, destMax: crate::types::size_t, src: *const ::core::ffi::c_void, count: crate::types::size_t) -> crate::types::errno_t;
    pub fn memset_s(dest: *mut ::core::ffi::c_void, destMax: crate::types::size_t, c: ::core::ffi::c_int, count: crate::types::size_t) -> crate::types::errno_t;
    pub fn snprintf_s(strDest: *mut ::core::ffi::c_char, destMax: crate::types::size_t, count: crate::types::size_t, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn AudioInfoCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemInfo: *mut crate::types::AudioCtrlElemInfo) -> i32;
    pub fn AudioInfoEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemInfo: *mut crate::types::AudioCtrlElemInfo) -> i32;
    pub fn AudioCodecGetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCodecSetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCodecGetEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCodecSetEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCodecSapmGetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCodecSapmSetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCodecSapmGetEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCodecSapmSetEnumCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCpuDaiGetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *mut crate::types::AudioCtrlElemValue) -> i32;
    pub fn AudioCpuDaiSetCtrlOps(kcontrol: *const crate::types::AudioKcontrol, elemValue: *const crate::types::AudioCtrlElemValue) -> i32;
    pub fn CopyFromUser(dest: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: u32) -> i32;
    pub fn CopyToUser(dest: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, count: u32) -> i32;
}
// === C2R_EXTERN_DECLS_END ===

// === C2R_ACCESSOR_SHIMS_BEGIN ===
// (auto-appended accessor shim declarations will be inserted here)
// === C2R_ACCESSOR_SHIMS_END ===
