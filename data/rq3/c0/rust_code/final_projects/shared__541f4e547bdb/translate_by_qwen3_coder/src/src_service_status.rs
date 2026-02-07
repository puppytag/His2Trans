//! Module: src_service_status
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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_service_status_1
// c_function: ServiceStatusMarshalling
// rust_file: src_service_status.rs
// rust_signature: pub extern "C" fn ServiceStatusMarshalling(status: *mut crate::types::ServiceStatus, buf: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int
// c_first_line: int ServiceStatusMarshalling(struct ServiceStatus *status, struct HdfSBuf *buf)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_service_status_1/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//     --> src/src_service_status.rs:27:77
//      |
//      |            -------------------------------------------------------------    ^ expected `bool`, found integer
//      |            |
//      |            expected because this is `bool`
//   error[E0308]: mismatched types
//     --> src/src_service_status.rs:28:81
// =================================
pub extern "C" fn ServiceStatusMarshalling(status: *mut crate::types::ServiceStatus, buf: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_service_status::ServiceStatusMarshalling(status as _, buf as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_service_status_1
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C0/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_service_status_1/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn ServiceStatusMarshalling(status: *mut crate::types::ServiceStatus, buf: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int {
    unsafe {
        if status.is_null() || buf.is_null() || (*status).serviceName.is_null() {
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
        
        let info_str = if (*status).info.is_null() {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            (*status).info
        };
        
        if crate::compat::HdfSbufWriteString(buf, (*status).serviceName) == 0
            || crate::compat::HdfSbufWriteUint16(buf, (*status).deviceClass) == 0
            || crate::compat::HdfSbufWriteUint16(buf, (*status).status) == 0
            || crate::compat::HdfSbufWriteString(buf, info_str) == 0
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to marshalling service status\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return crate::types::HDF_FAILURE;
        }
        
        crate::types::HDF_SUCCESS
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_service_status_1
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn ServiceStatusUnMarshalling(status: *mut crate::types::ServiceStatus, buf: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int {
    if status.is_null() || buf.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    
    unsafe {
        (*status).serviceName = crate::compat::HdfSbufReadString(buf);
        if (*status).serviceName.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to unmarshalling service status, service name is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return crate::types::HDF_FAILURE;
        }
        
        if crate::compat::HdfSbufReadUint16(buf, &mut (*status).deviceClass as *mut u16) == false
            || crate::compat::HdfSbufReadUint16(buf, &mut (*status).status as *mut u16) == false
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to unmarshalling service status, deviceClass or status invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return crate::types::HDF_FAILURE;
        }
        
        (*status).info = crate::compat::HdfSbufReadString(buf);
    }
    
    crate::types::HDF_SUCCESS
}
