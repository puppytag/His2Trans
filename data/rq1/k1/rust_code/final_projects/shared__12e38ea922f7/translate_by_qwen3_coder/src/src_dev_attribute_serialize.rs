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
        if !crate::compat::HdfSbufWriteUint32(sbuf, (*attribute).deviceId) ||
            !crate::compat::HdfSbufWriteUint16(sbuf, (*attribute).policy) ||
            !crate::compat::HdfSbufWriteString(sbuf, (*attribute).svcName) ||
            !crate::compat::HdfSbufWriteString(sbuf, (*attribute).moduleName) ||
            !crate::compat::HdfSbufWriteString(sbuf, (*attribute).deviceName)
        {
            return false;
        }
        if !(*attribute).deviceMatchAttr.is_null() {
            if !crate::compat::HdfSbufWriteUint32(sbuf, 1) ||
                !crate::compat::HdfSbufWriteString(sbuf, (*attribute).deviceMatchAttr)
            {
                return false;
            }
        } else {
            if !crate::compat::HdfSbufWriteUint32(sbuf, 0) {
                return false;
            }
        }
    }
    true
}

fn DeviceAttributeSet(attribute: *mut crate::types::HdfDeviceInfo, sbuf: *mut crate::types::HdfSBuf) -> bool {
    unsafe {
        let svcName = crate::compat::HdfSbufReadString(sbuf);
        if svcName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from sbuf failed, svcName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }
        (*attribute).svcName = libc::strdup(svcName);
        if (*attribute).svcName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from sbuf failed, strdup svcName fail\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }

        let moduleName = crate::compat::HdfSbufReadString(sbuf);
        if moduleName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from parcel failed, moduleName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }
        (*attribute).moduleName = libc::strdup(moduleName);
        if (*attribute).moduleName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from sbuf failed, strdup moduleName fail\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }

        let deviceName = crate::compat::HdfSbufReadString(sbuf);
        if deviceName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from sbuf failed, deviceName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }
        (*attribute).deviceName = libc::strdup(deviceName);
        if (*attribute).deviceName.is_null() {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Read from sbuf failed, strdup deviceName fail\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }

        let mut length: u32 = 0;
        if !crate::compat::HdfSbufReadUint32(sbuf, &mut length) {
            let _ = crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                b"Device attribute readDeviceMatchAttr length failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return false;
        }
        if length == 1 {
            let deviceMatchAttr = crate::compat::HdfSbufReadString(sbuf);
            if deviceMatchAttr.is_null() {
                let _ = crate::compat::HiLogPrint(
                    3u32,
                    3u32,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                    b"DeviceAttributeSet: Read from sbuf failed, deviceMatchAttr is null\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return false;
            }
            (*attribute).deviceMatchAttr = libc::strdup(deviceMatchAttr);
            if (*attribute).deviceMatchAttr.is_null() {
                let _ = crate::compat::HiLogPrint(
                    3u32,
                    3u32,
                    0xD002510,
                    b"dev_attr_serialze\0".as_ptr() as *const ::core::ffi::c_char,
                    b"Read from sbuf failed, strdup deviceMatchAttr fail\0".as_ptr() as *const ::core::ffi::c_char,
                );
                return false;
            }
        }

        true
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_dev_attribute_serialize_3
// c_function: DeviceAttributeDeserialize
// rust_file: src_dev_attribute_serialize.rs
// rust_signature: pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo
// c_first_line: struct HdfDeviceInfo *DeviceAttributeDeserialize(struct HdfSBuf *sbuf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk1-again/intermediate/shared__12e38ea922f7/workspace/repair_history/shared__12e38ea922f7/translate_by_qwen3_coder/_manual_fix/src_dev_attribute_serialize_3/translated_rust.rs
// last_error_truncated:
//   error: cannot find macro `LOG_CORE` in this scope
//      --> src/src_dev_attribute_serialize.rs:168:32
//       |
//       |                                ^^^^^^^^
//   error: cannot find macro `LOG_CORE` in this scope
//      --> src/src_dev_attribute_serialize.rs:165:32
//       |
//       |                                ^^^^^^^^
// =================================
pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_dev_attribute_serialize::DeviceAttributeDeserialize(sbuf as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_dev_attribute_serialize_3
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk1-again/intermediate/shared__12e38ea922f7/workspace/repair_history/shared__12e38ea922f7/translate_by_qwen3_coder/_manual_fix/src_dev_attribute_serialize_3/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn DeviceAttributeDeserialize(sbuf: *mut HdfSBuf) -> *mut HdfDeviceInfo {
    if sbuf.is_null() {
        return std::ptr::null_mut();
    }
    let attribute = HdfDeviceInfoNewInstance();
    if attribute.is_null() {
        let _ = HiLogPrint(LOG_CORE!(), LOG_ERROR, 0xD002510, "dev_attr_serialze\0".as_ptr() as *const _, "OsalMemCalloc failed, attribute is null\0".as_ptr() as *const _);
        return std::ptr::null_mut();
    }
    unsafe {
        if (*attribute).deviceMatchAttr.is_null() {
            let _ = HiLogPrint(LOG_CORE!(), LOG_WARN, 0xD002510, "dev_attr_serialze\0".as_ptr() as *const _, "OsalMemCalloc failed, attribute->deviceMatchAttr is null\0".as_ptr() as *const _);
        }
        if !HdfSbufReadUint32(sbuf, &mut (*attribute).deviceId) || !HdfSbufReadUint16(sbuf, &mut (*attribute).policy) {
            let _ = HiLogPrint(LOG_CORE!(), LOG_ERROR, 0xD002510, "dev_attr_serialze\0".as_ptr() as *const _, "invalid deviceId or policy\0".as_ptr() as *const _);
            crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
            return std::ptr::null_mut();
        }
        if crate::src_dev_attribute_serialize::DeviceAttributeSet(attribute, sbuf) {
            return attribute;
        }
        crate::src_dev_attribute_serialize::DeviceSerializedAttributeRelease(attribute);
    }
    std::ptr::null_mut()
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_dev_attribute_serialize_3
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn DeviceSerializedAttributeRelease(attribute: *mut HdfDeviceInfo) {
    if attribute.is_null() {
        return;
    }
    unsafe {
        if (*attribute).moduleName != std::ptr::null_mut() {
            OsalMemFree((*attribute).moduleName as *mut ::core::ffi::c_void);
        }
        if (*attribute).svcName != std::ptr::null_mut() {
            OsalMemFree((*attribute).svcName as *mut ::core::ffi::c_void);
        }
        if (*attribute).deviceName != std::ptr::null_mut() {
            OsalMemFree((*attribute).deviceName as *mut ::core::ffi::c_void);
        }
        if (*attribute).deviceMatchAttr != std::ptr::null_mut() {
            OsalMemFree((*attribute).deviceMatchAttr as *mut ::core::ffi::c_void);
        }
        OsalMemFree(attribute as *mut ::core::ffi::c_void);
    }
}
