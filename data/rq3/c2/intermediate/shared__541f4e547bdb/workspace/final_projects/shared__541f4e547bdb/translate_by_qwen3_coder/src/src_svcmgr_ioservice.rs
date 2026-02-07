//! Module: src_svcmgr_ioservice
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
// func_key: src_svcmgr_ioservice_1
// c_function: ProcessListenClass
// rust_file: src_svcmgr_ioservice.rs
// rust_signature: fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32
// c_first_line: static int32_t ProcessListenClass(struct SvcMgrIoservice *svcmgrInst, uint16_t devClass, int cmdId)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_1/translated_rust.rs
// last_error_truncated:
//   error[E0308]: arguments to this function are incorrect
//       --> src/src_svcmgr_ioservice.rs:40:19
//        |
//        |                   ^^^^^^^^^^^^^^^^^^^^
//        |
//       --> src/src_svcmgr_ioservice.rs:41:13
//        |
//        |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// =================================
fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_svcmgr_ioservice::ProcessListenClass(svcmgrInst as _, devClass as _, cmdId as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_svcmgr_ioservice_1
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_1/translated_rust.rs
 * ------------------------------------------------------------
fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32 {
    let data = unsafe { crate::compat::HdfSbufObtainDefaultSize() };
    if data.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    let _ = unsafe { crate::compat::HdfSbufWriteUint16(data, devClass) };

    unsafe {
        let iosvc = (*svcmgrInst).iosvc;
        if iosvc.is_null() {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        let dispatcher = (*iosvc).dispatcher;
        if dispatcher.is_null() {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        let dispatch_fn = (*dispatcher).Dispatch;
        if dispatch_fn.is_none() {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }

        let ret = dispatch_fn.unwrap()(
            iosvc as *mut crate::types::HdfObject,
            cmdId,
            data,
            std::ptr::null_mut(),
        );
        crate::compat::HdfSbufRecycle(data);
        ret
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_svcmgr_ioservice_1
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn SetListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16) -> i32 {
    crate::src_svcmgr_ioservice::ProcessListenClass(svcmgrInst, devClass, crate::types::SVCMGR_REGISTER_LISTENER as i32)
}

fn UnSetListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16) -> i32 {
    crate::src_svcmgr_ioservice::ProcessListenClass(svcmgrInst, devClass, crate::types::SVCMGR_UNREGISTER_LISTENER as i32)
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_svcmgr_ioservice_4
// c_function: SvcMgrIoserviceRegSvcStatListener
// rust_file: src_svcmgr_ioservice.rs
// rust_signature: pub extern "C" fn SvcMgrIoserviceRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener, deviceClass: u16) -> i32
// c_first_line: int32_t SvcMgrIoserviceRegSvcStatListener(
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_4/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_svcmgr_ioservice.rs:128:13
//        |
//        |         --------------------------------------------- arguments to this function are incorrect
//        |             ^^^^^^^^^^^^^^^^^^^ expected `types::HdfIoService`, found `HdfIoService`
//        |
//       --> src/__c2r_generated/tu_types_src_svcmgr_ioservice.rs:67:1
//        |
// =================================
pub extern "C" fn SvcMgrIoserviceRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener, deviceClass: u16) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_svcmgr_ioservice::SvcMgrIoserviceRegSvcStatListener(self_ as _, listener as _, deviceClass as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_svcmgr_ioservice_4
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_4/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn SvcMgrIoserviceRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener, deviceClass: u16) -> i32 {
    if self_.is_null() || listener.is_null() || deviceClass as u32 >= crate::types::DEVICE_CLASS_MAX {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr)
    let svcmgr_offset = unsafe {
        &(*(std::ptr::null::<crate::types::SvcMgrIoservice>())).svcmgr as *const _ as usize
    };
    let svcmgrInst = (self_ as *mut u8).wrapping_sub(svcmgr_offset) as *mut crate::types::SvcMgrIoservice;

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstatListener_offset = unsafe {
        &(*(std::ptr::null::<crate::types::IoServiceStatusListener>())).svcstatListener as *const _ as usize
    };
    let listenerInst = (listener as *mut u8).wrapping_sub(svcstatListener_offset) as *mut crate::types::IoServiceStatusListener;

    unsafe {
        (*listenerInst).deviceClass = deviceClass;
    }

    let ret = crate::src_svcmgr_ioservice::SetListenClass(svcmgrInst, deviceClass);
    if ret != crate::types::HDF_SUCCESS {
        // HDF_LOGE("failed to set listen class");
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to set listen class\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return ret;
    }

    unsafe {
        crate::compat::HdfDeviceRegisterEventListener(
            (*svcmgrInst).iosvc,
            &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener,
        )
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_svcmgr_ioservice_4
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_svcmgr_ioservice_5
// c_function: SvcMgrIoserviceUnRegSvcStatListener
// rust_file: src_svcmgr_ioservice.rs
// rust_signature: pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32
// c_first_line: int32_t SvcMgrIoserviceUnRegSvcStatListener(struct ISvcMgrIoservice *self, struct ServiceStatusListener *listener)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_5/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_svcmgr_ioservice.rs:189:76
//        |
//        |                            ----------------------------------------------- ^^^^^ expected `types::HdfIoService`, found `HdfIoService`
//        |                            |
//        |                            arguments to this function are incorrect
//        |
//       --> src/__c2r_generated/tu_types_src_svcmgr_ioservice.rs:67:1
// =================================
pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_svcmgr_ioservice::SvcMgrIoserviceUnRegSvcStatListener(self_ as _, listener as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_svcmgr_ioservice_5
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_5/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr)
    let svcmgr_offset = unsafe {
        &(*(std::ptr::null::<crate::types::SvcMgrIoservice>())).svcmgr as *const _ as usize
    };
    let svcmgrInst = unsafe {
        (self_ as *mut u8).sub(svcmgr_offset) as *mut crate::types::SvcMgrIoservice
    };

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstatListener_offset = unsafe {
        &(*(std::ptr::null::<crate::types::IoServiceStatusListener>())).svcstatListener as *const _ as usize
    };
    let listenerInst = unsafe {
        (listener as *mut u8).sub(svcstatListener_offset) as *mut crate::types::IoServiceStatusListener
    };

    let iosvc = unsafe { (*svcmgrInst).iosvc };
    let ioservListener_ptr = unsafe { &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener };

    let mut ret = unsafe { crate::compat::HdfDeviceUnregisterEventListener(iosvc, ioservListener_ptr) };
    if ret != crate::types::HDF_SUCCESS as i32 {
        return ret;
    }

    if unsafe { crate::compat::HdfIoserviceGetListenerCount(iosvc as *const _) } == 0 {
        let deviceClass = unsafe { (*listenerInst).deviceClass };
        ret = crate::src_svcmgr_ioservice::UnSetListenClass(svcmgrInst, deviceClass);
    }

    ret
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_svcmgr_ioservice_5
 * === C2R_LLM_FAILED_OUTPUT_END === */


fn SvcMgrIoserviceConstruct(svcmgrInst: *mut crate::types::ISvcMgrIoservice) {
    unsafe {
        (*svcmgrInst).RegisterServiceStatusListener = Some(SvcMgrIoserviceRegSvcStatListener);
        (*svcmgrInst).UnregisterServiceStatusListener = Some(SvcMgrIoserviceUnRegSvcStatListener);
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_svcmgr_ioservice_7
// c_function: SvcMgrIoserviceGet
// rust_file: src_svcmgr_ioservice.rs
// rust_signature: pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice
// c_first_line: struct ISvcMgrIoservice *SvcMgrIoserviceGet(void)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_7/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_svcmgr_ioservice.rs:134:32
//        |
//        |         --------------------   ^^^^^ expected `HdfIoService`, found `types::HdfIoService`
//        |         |
//        |         expected due to the type of this binding
//        |
//       --> src/types.rs:1216:1
// =================================
pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_svcmgr_ioservice::SvcMgrIoserviceGet() as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_svcmgr_ioservice_7
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_7/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice {
    unsafe {
        let svcmgr_inst = crate::compat::OsalMemCalloc(
            std::mem::size_of::<crate::types::SvcMgrIoservice>() as u32
        ) as *mut crate::types::SvcMgrIoservice;
        
        if svcmgr_inst.is_null() {
            return std::ptr::null_mut();
        }
        
        let service_name = b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char;
        let iosvc = crate::src_hdf_io_service::HdfIoServiceBind(service_name);
        
        if iosvc.is_null() {
            let tag = b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char;
            let fmt = b"ioserivce %{public}s not exist\0".as_ptr() as *const ::core::ffi::c_char;
            let svc_name_arg = b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char;
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                tag,
                fmt,
                svc_name_arg,
            );
            crate::compat::OsalMemFree(svcmgr_inst as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        (*svcmgr_inst).iosvc = iosvc;
        
        let svcmgr_ptr = &mut (*svcmgr_inst).svcmgr as *mut crate::types::ISvcMgrIoservice;
        crate::src_svcmgr_ioservice::SvcMgrIoserviceConstruct(svcmgr_ptr);
        
        svcmgr_ptr
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_svcmgr_ioservice_7
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_0
// func_key: src_svcmgr_ioservice_8
// c_function: SvcMgrIoserviceRelease
// rust_file: src_svcmgr_ioservice.rs
// rust_signature: pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice)
// c_first_line: void SvcMgrIoserviceRelease(struct ISvcMgrIoservice *svcmgr)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_8/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_svcmgr_ioservice.rs:189:56
//        |
//        |         ---------------------------------------------- ^^^^^^^^^^^^^^^^^^^^ expected `types::HdfIoService`, found `HdfIoService`
//        |         |
//        |         arguments to this function are incorrect
//        |
//       --> src/__c2r_generated/tu_types_src_svcmgr_ioservice.rs:67:1
// =================================
pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_svcmgr_ioservice::SvcMgrIoserviceRelease(svcmgr as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_svcmgr_ioservice_8
 * reason: repair_failed_after_0
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_C2/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_8/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    
    // CONTAINER_OF macro: get SvcMgrIoservice from ISvcMgrIoservice pointer
    // offset = &((struct SvcMgrIoservice *)0)->svcmgr
    let offset = unsafe {
        &(*(std::ptr::null::<crate::types::SvcMgrIoservice>())).svcmgr as *const _ as usize
    };
    let svcmgr_inst = unsafe {
        (svcmgr as *mut u8).sub(offset) as *mut crate::types::SvcMgrIoservice
    };
    
    unsafe {
        crate::src_hdf_io_service::HdfIoServiceRecycle((*svcmgr_inst).iosvc);
        crate::compat::OsalMemFree(svcmgr_inst as *mut ::core::ffi::c_void);
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_svcmgr_ioservice_8
 * === C2R_LLM_FAILED_OUTPUT_END === */

