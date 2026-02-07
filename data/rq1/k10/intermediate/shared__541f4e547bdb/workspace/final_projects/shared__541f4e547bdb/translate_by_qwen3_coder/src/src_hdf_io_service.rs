//! Module: src_hdf_io_service
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

pub extern "C" fn HdfIoServiceBind(serviceName: *const ::core::ffi::c_char) -> *mut crate::types::HdfIoService {
    unsafe { crate::compat::HdfIoServiceAdapterObtain(serviceName) }
}

pub extern "C" fn HdfIoServiceRecycle(service: *mut crate::types::HdfIoService) {
    unsafe {
        crate::compat::HdfIoServiceAdapterRecycle(service);
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_hdf_io_service_3
// c_function: HdfIoServicePublish
// rust_file: src_hdf_io_service.rs
// rust_signature: pub extern "C" fn HdfIoServicePublish(serviceName: *const ::core::ffi::c_char, mode: u32) -> *mut crate::types::HdfIoService
// c_first_line: struct HdfIoService *HdfIoServicePublish(const char *serviceName, uint32_t mode)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_hdf_io_service_3/translated_rust.rs
// last_error_truncated:
//   error[E0609]: no field `is_null` on type `unsafe extern "C" fn(*const i8, u32) -> *mut types::HdfIoService {compat::HdfIoServiceAdapterPublish}`
//     --> src/src_hdf_io_service.rs:27:41
//      |
//      |                                         ^^^^^^^ unknown field
//   error: could not compile `shared__541f4e547bdb` (bin "shared__541f4e547bdb") due to 1 previous error
// =================================
pub extern "C" fn HdfIoServicePublish(serviceName: *const ::core::ffi::c_char, mode: u32) -> *mut crate::types::HdfIoService {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_hdf_io_service::HdfIoServicePublish(serviceName as _, mode as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_hdf_io_service_3
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_hdf_io_service_3/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfIoServicePublish(serviceName: *const ::core::ffi::c_char, mode: u32) -> *mut crate::types::HdfIoService {
    unsafe {
        if !(HdfIoServiceAdapterPublish.is_null)() {
            return HdfIoServiceAdapterPublish(serviceName, mode);
        }
        std::ptr::null_mut()
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_hdf_io_service_3
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_hdf_io_service_4
// c_function: HdfIoServiceRemove
// rust_file: src_hdf_io_service.rs
// rust_signature: pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService)
// c_first_line: void HdfIoServiceRemove(struct HdfIoService *service)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_hdf_io_service_4/translated_rust.rs
// last_error_truncated:
//   error[E0599]: no method named `is_null` found for fn item `unsafe extern "C" fn(*mut types::HdfIoService) {compat::HdfIoServiceAdapterRemove}` in the current scope
//     --> src/src_hdf_io_service.rs:64:54
//      |
//      |                                                      ^^^^^^^ method not found in `unsafe extern "C" fn(*mut types::HdfIoService) {compat::HdfIoServiceAdapterRemove}`
//   error: could not compile `shared__541f4e547bdb` (bin "shared__541f4e547bdb") due to 1 previous error
// =================================
pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_hdf_io_service::HdfIoServiceRemove(service as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_hdf_io_service_4
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_hdf_io_service_4/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    unsafe {
        if !crate::compat::HdfIoServiceAdapterRemove.is_null() {
            crate::compat::HdfIoServiceAdapterRemove(service);
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_hdf_io_service_4
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn HdfIoServiceDispatch(ioService: *mut crate::types::HdfIoService, cmdId: ::core::ffi::c_int, data: *mut crate::types::HdfSBuf, reply: *mut crate::types::HdfSBuf) -> i32 {
    if ioService.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    unsafe {
        let dispatcher = (*ioService).dispatcher;
        if dispatcher.is_null() {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        let dispatch_fn = (*dispatcher).Dispatch;
        if dispatch_fn.is_none() {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        let service_ptr = &mut (*ioService).object as *mut crate::types::HdfObject;
        dispatch_fn.unwrap()(service_ptr, cmdId, data, reply)
    }
}
