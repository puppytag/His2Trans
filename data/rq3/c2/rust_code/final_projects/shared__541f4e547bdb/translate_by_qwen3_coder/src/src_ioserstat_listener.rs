//! Module: src_ioserstat_listener
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

fn OnIoServiceEventReceive(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    if listener.is_null() || service.is_null() || data.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let _ = id;
    
    let mut status = crate::types::ServiceStatus {
        serviceName: std::ptr::null(),
        deviceClass: 0,
        status: 0,
        info: std::ptr::null(),
    };
    
    if crate::src_service_status::ServiceStatusUnMarshalling(&mut status as *mut crate::types::ServiceStatus, data) != crate::types::HDF_SUCCESS {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    
    unsafe {
        let status_listener = (*listener).priv_ as *mut crate::types::IoServiceStatusListener;
        if let Some(callback) = (*status_listener).svcstatListener.callback {
            if ((*status_listener).deviceClass & status.deviceClass) != 0 {
                callback(
                    &mut (*status_listener).svcstatListener as *mut crate::types::ServiceStatusListener,
                    &mut status as *mut crate::types::ServiceStatus,
                );
            }
        }
    }
    
    crate::types::HDF_SUCCESS
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_ioserstat_listener_2
// c_function: IoServiceStatusListenerNewInstance
// rust_file: src_ioserstat_listener.rs
// rust_signature: pub extern "C" fn IoServiceStatusListenerNewInstance() -> *mut crate::types::ServiceStatusListener
// c_first_line: struct ServiceStatusListener *IoServiceStatusListenerNewInstance(void)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_ioserstat_listener_2/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//      --> src/src_ioserstat_listener.rs:29:53
//       |
//       |                                                ---- ^^^^^^^^^^^^^^^^^^^^^^^ expected "C" fn, found "Rust" fn
//       |                                                |
//       |                                                arguments to this enum variant are incorrect
//       |
//   help: the type constructed contains `fn(*mut HdfDevEventlistener, *mut types::HdfIoService, u32, *mut types::HdfSBuf) -> i32 {OnIoServiceEventReceive}` due to the type of the argument passed
// =================================
pub extern "C" fn IoServiceStatusListenerNewInstance() -> *mut crate::types::ServiceStatusListener {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_ioserstat_listener::IoServiceStatusListenerNewInstance() as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_ioserstat_listener_2
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_ioserstat_listener_2/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn IoServiceStatusListenerNewInstance() -> *mut crate::types::ServiceStatusListener {
    let listener = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::IoServiceStatusListener>() as u32)
    } as *mut crate::types::IoServiceStatusListener;
    
    if listener.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        (*listener).ioservListener.onReceive = Some(OnIoServiceEventReceive);
        (*listener).ioservListener.priv_ = listener as *mut ::core::ffi::c_void;
        
        &mut (*listener).svcstatListener as *mut crate::types::ServiceStatusListener
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_ioserstat_listener_2
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_ioserstat_listener_3
// c_function: IoServiceStatusListenerFree
// rust_file: src_ioserstat_listener.rs
// rust_signature: pub extern "C" fn IoServiceStatusListenerFree(listener: *mut crate::types::ServiceStatusListener)
// c_first_line: void IoServiceStatusListenerFree(struct ServiceStatusListener *listener)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_ioserstat_listener_3/translated_rust.rs
// last_error_truncated:
//   error: dereferencing a null pointer
//     --> src/src_ioserstat_listener.rs:74:10
//      |
//      |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this code causes undefined behavior when executed
//      |
//   error: could not compile `shared__541f4e547bdb` (bin "shared__541f4e547bdb") due to 1 previous error; 5 warnings emitted
// =================================
pub extern "C" fn IoServiceStatusListenerFree(listener: *mut crate::types::ServiceStatusListener) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_ioserstat_listener::IoServiceStatusListenerFree(listener as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_ioserstat_listener_3
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_ioserstat_listener_3/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn IoServiceStatusListenerFree(listener: *mut crate::types::ServiceStatusListener) {
    if listener.is_null() {
        return;
    }
    // CONTAINER_OF macro: (char*)(listener) - (char*)&((struct IoServiceStatusListener*)0)->svcstatListener
    // This computes the offset of svcstatListener within IoServiceStatusListener and subtracts it from listener
    let offset = unsafe {
        &(*(std::ptr::null::<crate::types::IoServiceStatusListener>())).svcstatListener as *const _ as usize
    };
    let ioserv_listener = (listener as *mut u8).wrapping_sub(offset) as *mut crate::types::IoServiceStatusListener;
    unsafe {
        crate::compat::OsalMemFree(ioserv_listener as *mut ::core::ffi::c_void);
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_ioserstat_listener_3
 * === C2R_LLM_FAILED_OUTPUT_END === */

