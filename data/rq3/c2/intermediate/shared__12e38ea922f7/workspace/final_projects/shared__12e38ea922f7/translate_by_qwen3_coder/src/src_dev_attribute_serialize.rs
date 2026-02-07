//! Module: src_dev_attribute_serialize
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

pub extern "C" fn DeviceAttributeSerialize(attribute: *const HdfDeviceInfo, sbuf: *mut HdfSBuf) -> bool {
    if attribute.is_null() || sbuf.is_null() {
        return false;
    }

    unsafe {
        if !HdfSbufWriteUint32(sbuf, (*attribute).deviceId) ||
           !HdfSbufWriteUint16(sbuf, (*attribute).policy) ||
           !HdfSbufWriteString(sbuf, (*attribute).svcName) ||
           !HdfSbufWriteString(sbuf, (*attribute).moduleName) ||
           !HdfSbufWriteString(sbuf, (*attribute).deviceName) {
            return false;
        }

        if !(*attribute).deviceMatchAttr.is_null() {
            if !HdfSbufWriteUint32(sbuf, 1) ||
               !HdfSbufWriteString(sbuf, (*attribute).deviceMatchAttr) {
                return false;
            }
        } else {
            if !HdfSbufWriteUint32(sbuf, 0) {
                return false;
            }
        }
    }

    true
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_dev_attribute_serialize_2
// c_function: DeviceAttributeSet
// rust_file: src_dev_attribute_serialize.rs
// rust_signature: fn DeviceAttributeSet(attribute: *mut crate::types::HdfDeviceInfo, sbuf: *mut crate::types::HdfSBuf) -> bool
// c_first_line: static _Bool DeviceAttributeSet(struct HdfDeviceInfo *attribute, struct HdfSBuf *sbuf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__12e38ea922f7/workspace/repair_history/shared__12e38ea922f7/translate_by_qwen3_coder/_manual_fix/src_dev_attribute_serialize_2/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `LOG_CORE` in this scope
//     --> src/src_dev_attribute_serialize.rs:49:17
//      |
//      |                 ^^^^^^^^ not found in this scope
//   error[E0425]: cannot find value `LOG_ERROR` in this scope
//     --> src/src_dev_attribute_serialize.rs:50:17
//      |
//      |                 ^^^^^^^^^ not found in this scope
// =================================
fn DeviceAttributeSet(attribute: *mut crate::types::HdfDeviceInfo, sbuf: *mut crate::types::HdfSBuf) -> bool {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_dev_attribute_serialize::DeviceAttributeSet(attribute as _, sbuf as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_dev_attribute_serialize_2
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__12e38ea922f7/workspace/repair_history/shared__12e38ea922f7/translate_by_qwen3_coder/_manual_fix/src_dev_attribute_serialize_2/translated_rust.rs
 * ------------------------------------------------------------
fn DeviceAttributeSet(attribute: *mut crate::types::HdfDeviceInfo, sbuf: *mut crate::types::HdfSBuf) -> bool {
    unsafe {
        let svcName = HdfSbufReadString(sbuf);
        if svcName.is_null() {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from sbuf failed, svcName is null\0".as_ptr() as *const i8,
            );
            return false;
        }
        (*attribute).svcName = libc::strdup(svcName);
        if (*attribute).svcName.is_null() {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from sbuf failed, strdup svcName fail\0".as_ptr() as *const i8,
            );
            return false;
        }

        let moduleName = HdfSbufReadString(sbuf);
        if moduleName.is_null() {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from parcel failed, moduleName is null\0".as_ptr() as *const i8,
            );
            return false;
        }
        (*attribute).moduleName = libc::strdup(moduleName);
        if (*attribute).moduleName.is_null() {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from sbuf failed, strdup moduleName fail\0".as_ptr() as *const i8,
            );
            return false;
        }

        let deviceName = HdfSbufReadString(sbuf);
        if deviceName.is_null() {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from sbuf failed, deviceName is null\0".as_ptr() as *const i8,
            );
            return false;
        }
        (*attribute).deviceName = libc::strdup(deviceName);
        if (*attribute).deviceName.is_null() {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Read from sbuf failed, strdup deviceName fail\0".as_ptr() as *const i8,
            );
            return false;
        }

        let mut length: u32 = 0;
        if HdfSbufReadUint32(sbuf, &mut length) == false {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const i8,
                b"Device attribute readDeviceMatchAttr length failed\0".as_ptr() as *const i8,
            );
            return false;
        }
        if length == 1 {
            let deviceMatchAttr = HdfSbufReadString(sbuf);
            if deviceMatchAttr.is_null() {
                let _ = HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const i8,
                    b"DeviceAttributeSet: Read from sbuf failed, deviceMatchAttr is null\0".as_ptr() as *const i8,
                );
                return false;
            }
            (*attribute).deviceMatchAttr = libc::strdup(deviceMatchAttr);
            if (*attribute).deviceMatchAttr.is_null() {
                let _ = HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const i8,
                    b"Read from sbuf failed, strdup deviceMatchAttr fail\0".as_ptr() as *const i8,
                );
                return false;
            }
        }

        true
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_dev_attribute_serialize_2
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_dev_attribute_serialize_3
// c_function: DeviceAttributeDeserialize
// rust_file: src_dev_attribute_serialize.rs
// rust_signature: pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo
// c_first_line: struct HdfDeviceInfo *DeviceAttributeDeserialize(struct HdfSBuf *sbuf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__12e38ea922f7/workspace/repair_history/shared__12e38ea922f7/translate_by_qwen3_coder/_manual_fix/src_dev_attribute_serialize_3/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `LOG_CORE` in this scope
//      --> src/src_dev_attribute_serialize.rs:195:17
//       |
//       |                 ^^^^^^^^ not found in this scope
//       |
//   help: consider importing one of these constants
//       |
//       |
// =================================
pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_dev_attribute_serialize::DeviceAttributeDeserialize(sbuf as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_dev_attribute_serialize_3
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__12e38ea922f7/workspace/repair_history/shared__12e38ea922f7/translate_by_qwen3_coder/_manual_fix/src_dev_attribute_serialize_3/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo {
    if sbuf.is_null() {
        return std::ptr::null_mut();
    }

    let attribute = unsafe { HdfDeviceInfoNewInstance() };
    if attribute.is_null() {
        let _ = unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"OsalMemCalloc failed, attribute is null\0".as_ptr() as *const ::core::ffi::c_char,
            )
        };
        return std::ptr::null_mut();
    }

    unsafe {
        if (*attribute).deviceMatchAttr.is_null() {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_WARN,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"OsalMemCalloc failed, attribute->deviceMatchAttr is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }

        if !HdfSbufReadUint32(sbuf, &mut (*attribute).deviceId) || !HdfSbufReadUint16(sbuf, &mut (*attribute).policy) {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"invalid deviceId or policy\0".as_ptr() as *const ::core::ffi::c_char,
            );
            crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
            return std::ptr::null_mut();
        }

        if crate::src_dev_attribute_serialize::DeviceAttributeSet(attribute, sbuf) {
            return attribute;
        }

        crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
        std::ptr::null_mut()
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_dev_attribute_serialize_3
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn DeviceSerializedAttributeRelease(attribute: *mut HdfDeviceInfo) {
    if attribute.is_null() {
        return;
    }

    unsafe {
        if !(*attribute).moduleName.is_null() {
            OsalMemFree((*attribute).moduleName as *mut ::core::ffi::c_void);
        }
        if !(*attribute).svcName.is_null() {
            OsalMemFree((*attribute).svcName as *mut ::core::ffi::c_void);
        }
        if !(*attribute).deviceName.is_null() {
            OsalMemFree((*attribute).deviceName as *mut ::core::ffi::c_void);
        }
        if !(*attribute).deviceMatchAttr.is_null() {
            OsalMemFree((*attribute).deviceMatchAttr as *mut ::core::ffi::c_void);
        }
        OsalMemFree(attribute as *mut ::core::ffi::c_void);
    }
}
