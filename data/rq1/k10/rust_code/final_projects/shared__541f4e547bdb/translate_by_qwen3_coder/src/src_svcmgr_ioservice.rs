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
// reason: repair_failed_after_5
// func_key: src_svcmgr_ioservice_1
// c_function: ProcessListenClass
// rust_file: src_svcmgr_ioservice.rs
// rust_signature: fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32
// c_first_line: static int32_t ProcessListenClass(struct SvcMgrIoservice *svcmgrInst, uint16_t devClass, int cmdId)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_1/translated_rust.rs
// last_error_truncated:
//   error[E0308]: arguments to this function are incorrect
//       --> src/src_svcmgr_ioservice.rs:30:13
//        |
//        |             ^
//        |
//       --> src/src_svcmgr_ioservice.rs:30:15
//        |
//        |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// =================================
fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_svcmgr_ioservice::ProcessListenClass(svcmgrInst as _, devClass as _, cmdId as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_svcmgr_ioservice_1
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_1/translated_rust.rs
 * ------------------------------------------------------------
fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32 {
    let data = crate::compat::HdfSbufObtainDefaultSize();
    if data.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL as i32;
    }
    let _ = crate::compat::HdfSbufWriteUint16(data, devClass);
    unsafe {
        if (*svcmgrInst).iosvc.is_null()
            || (*(*svcmgrInst).iosvc).dispatcher.is_null()
            || (*(*(*svcmgrInst).iosvc).dispatcher).Dispatch.is_none()
        {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT as i32;
        }
        let ret = if let Some(f) = (*(*(*svcmgrInst).iosvc).dispatcher).Dispatch {
            f((*svcmgrInst).iosvc as *mut crate::types::HdfObject, cmdId, data, std::ptr::null_mut())
        } else {
            0
        };
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

pub extern "C" fn SvcMgrIoserviceRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener, deviceClass: u16) -> i32 {
    if self_.is_null() || listener.is_null() || deviceClass as u32 >= crate::types::DEVICE_CLASS_MAX {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let svcmgrInst = unsafe {
        (self_ as *mut u8).offset(-(std::mem::size_of::<crate::types::SvcMgrIoservice>() as isize)) as *mut crate::types::SvcMgrIoservice
    };
    let listenerInst = unsafe {
        (listener as *mut u8).offset(-(std::mem::size_of::<crate::types::IoServiceStatusListener>() as isize)) as *mut crate::types::IoServiceStatusListener
    };
    unsafe {
        (*listenerInst).deviceClass = deviceClass;
    }
    let ret = crate::src_svcmgr_ioservice::SetListenClass(svcmgrInst, deviceClass);
    if ret != crate::types::HDF_SUCCESS {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD002510, b"HDF_LOG_TAG\0".as_ptr() as *const i8, b"failed to set listen class\0".as_ptr() as *const i8);
        }
        return ret;
    }
    unsafe {
        crate::compat::HdfDeviceRegisterEventListener((*svcmgrInst).iosvc as *mut crate::types::HdfIoService, &mut (*listenerInst).ioservListener)
    }
}

pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }
    let svcmgrInst = unsafe {
        (self_ as *mut u8).offset(-(std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr) as isize))
            as *mut crate::types::SvcMgrIoservice
    };
    let listenerInst = unsafe {
        (listener as *mut u8).offset(-(std::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener) as isize))
            as *mut crate::types::IoServiceStatusListener
    };
    let ret = unsafe {
        crate::compat::HdfDeviceUnregisterEventListener(
            (*svcmgrInst).iosvc as *mut crate::types::HdfIoService,
            &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener,
        )
    };
    if ret != crate::types::HDF_SUCCESS as i32 {
        return ret;
    }
    let count = unsafe { crate::compat::HdfIoserviceGetListenerCount((*svcmgrInst).iosvc as *const crate::types::HdfIoService) };
    if count == 0 {
        let dev_class = unsafe { (*listenerInst).deviceClass };
        return crate::src_svcmgr_ioservice::UnSetListenClass(svcmgrInst, dev_class);
    }
    ret
}

fn SvcMgrIoserviceConstruct(svcmgrInst: *mut crate::types::ISvcMgrIoservice) {
    unsafe {
        (*svcmgrInst).RegisterServiceStatusListener = Some(SvcMgrIoserviceRegSvcStatListener);
        (*svcmgrInst).UnregisterServiceStatusListener = Some(SvcMgrIoserviceUnRegSvcStatListener);
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_svcmgr_ioservice_7
// c_function: SvcMgrIoserviceGet
// rust_file: src_svcmgr_ioservice.rs
// rust_signature: pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice
// c_first_line: struct ISvcMgrIoservice *SvcMgrIoserviceGet(void)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_7/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_svcmgr_ioservice.rs:102:31
//        |
//        |         -------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `__c2r_tu_types_src_svcmgr_ioservice::HdfIoService`, found `types::HdfIoService`
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
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/shared__541f4e547bdb/workspace/repair_history/shared__541f4e547bdb/translate_by_qwen3_coder/_manual_fix/src_svcmgr_ioservice_7/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice {
    let svcmgrInst = unsafe { libc::malloc(std::mem::size_of::<crate::types::SvcMgrIoservice>()) as *mut crate::types::SvcMgrIoservice };
    if svcmgrInst.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        std::ptr::write_bytes(svcmgrInst, 0, 1);
    }
    unsafe {
        (*svcmgrInst).iosvc = crate::src_hdf_io_service::HdfIoServiceBind(b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char);
    }
    unsafe {
        if (*svcmgrInst).iosvc.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"ioserivce %{public}s not exist\0".as_ptr() as *const ::core::ffi::c_char,
                b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            libc::free(svcmgrInst as *mut libc::c_void);
            return std::ptr::null_mut();
        }
    }
    unsafe {
        crate::src_svcmgr_ioservice::SvcMgrIoserviceConstruct(std::ptr::addr_of_mut!((*svcmgrInst).svcmgr) as *mut crate::types::ISvcMgrIoservice);
    }
    unsafe {
        std::ptr::addr_of_mut!((*svcmgrInst).svcmgr) as *mut crate::types::ISvcMgrIoservice
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_svcmgr_ioservice_7
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    let svcmgr_inst = unsafe { (svcmgr as *mut u8).offset(-(std::mem::size_of::<crate::types::ISvcMgrIoservice>() as isize)) } as *mut crate::types::SvcMgrIoservice;
    unsafe {
        crate::src_hdf_io_service::HdfIoServiceRecycle((*svcmgr_inst).iosvc as *mut crate::types::HdfIoService);
        libc::free(svcmgr_inst as *mut std::ffi::c_void);
    }
}
