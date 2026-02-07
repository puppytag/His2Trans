//! Module: src_devmgr_service_clnt
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

pub extern "C" fn DevmgrServiceClntAttachDeviceHost(hostId: u16, hostService: *mut crate::types::IDevHostService) -> ::core::ffi::c_int {
    let mut devMgrSvcIf: *mut crate::types::IDevmgrService = std::ptr::null_mut();
    let inst = crate::src_devmgr_service_clnt::DevmgrServiceClntGetInstance();
    if inst.is_null() || unsafe { (*inst).devMgrSvcIf }.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devmgr_service_clnt\0".as_ptr() as *const _,
                b"failed to attach device host, get device manager service client is null\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    devMgrSvcIf = unsafe { (*inst).devMgrSvcIf };
    if unsafe { (*devMgrSvcIf).AttachDeviceHost }.is_none() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devmgr_service_clnt\0".as_ptr() as *const _,
                b"failed to attach device host, attach device host function is null\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        if let Some(f) = (*devMgrSvcIf).AttachDeviceHost {
            f(devMgrSvcIf, hostId, hostService)
        } else {
            crate::types::HDF_FAILURE
        }
    }
}

pub extern "C" fn DevmgrServiceClntAttachDevice(deviceToken: *mut crate::types::IHdfDeviceToken) -> ::core::ffi::c_int {
    let mut devMgrSvcIf: *mut crate::types::IDevmgrService = std::ptr::null_mut();
    let inst = crate::src_devmgr_service_clnt::DevmgrServiceClntGetInstance();
    if inst.is_null() || unsafe { (*inst).devMgrSvcIf.is_null() } {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devmgr_service_clnt\0".as_ptr() as *const _,
                b"devmgr client failed to attach device, inst is null\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    devMgrSvcIf = unsafe { (*inst).devMgrSvcIf };
    if unsafe { (*devMgrSvcIf).AttachDevice.is_none() } {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devmgr_service_clnt\0".as_ptr() as *const _,
                b"devmgr client failed to attach device, dmsOps->AttachDevice is nul\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        if let Some(f) = (*devMgrSvcIf).AttachDevice {
            f(devMgrSvcIf, deviceToken)
        } else {
            crate::types::HDF_FAILURE
        }
    }
}

pub extern "C" fn DevmgrServiceClntDetachDevice(devid: crate::types::devid_t) -> ::core::ffi::c_int {
    let mut devMgrSvcIf: *mut crate::types::IDevmgrService = std::ptr::null_mut();
    let inst = crate::src_devmgr_service_clnt::DevmgrServiceClntGetInstance();
    if inst.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"devmgr_service_clnt\0".as_ptr() as *const _,
            b"devmgr client failed to deatch device, inst is null\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        if (*inst).devMgrSvcIf.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devmgr_service_clnt\0".as_ptr() as *const _,
                b"devmgr client failed to deatch device, inst is null\0".as_ptr() as *const _,
            );
            return crate::types::HDF_FAILURE;
        }
        devMgrSvcIf = (*inst).devMgrSvcIf;
        if let Some(f) = (*devMgrSvcIf).DetachDevice {
            f(devMgrSvcIf, devid)
        } else {
            crate::types::HDF_FAILURE
        }
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_devmgr_service_clnt_4
// c_function: DevmgrServiceClntGetInstance
// rust_file: src_devmgr_service_clnt.rs
// rust_signature: pub extern "C" fn DevmgrServiceClntGetInstance() -> *mut crate::types::DevmgrServiceClnt
// c_first_line: struct DevmgrServiceClnt *DevmgrServiceClntGetInstance(void)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_devmgr_service_clnt_4/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `INSTANCE` in this scope
//     --> src/src_devmgr_service_clnt.rs:29:12
//      |
//      |            ^^^^^^^^ not found in this scope
//   error[E0425]: cannot find value `INSTANCE` in this scope
//     --> src/src_devmgr_service_clnt.rs:30:13
//      |
//      |             ^^^^^^^^ not found in this scope
// =================================
pub extern "C" fn DevmgrServiceClntGetInstance() -> *mut crate::types::DevmgrServiceClnt {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_devmgr_service_clnt::DevmgrServiceClntGetInstance() as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_devmgr_service_clnt_4
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk10-again/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_devmgr_service_clnt_4/translated_rust.rs
 * ------------------------------------------------------------
static mut INSTANCE: crate::types::DevmgrServiceClnt = crate::types::DevmgrServiceClnt {
    devMgrSvcIf: std::ptr::null_mut(),
};

pub extern "C" fn DevmgrServiceClntGetInstance() -> *mut crate::types::DevmgrServiceClnt {
    unsafe {
        if INSTANCE.devMgrSvcIf.is_null() {
            INSTANCE.devMgrSvcIf = crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVMGR_SERVICE as i32) as *mut crate::types::IDevmgrService;
        }
        &mut INSTANCE
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_devmgr_service_clnt_4
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn DevmgrServiceClntFreeInstance(inst: *mut crate::types::DevmgrServiceClnt) {
    if !inst.is_null() {
        unsafe {
            let dev_mgr_svc_if = (*inst).devMgrSvcIf;
            if !dev_mgr_svc_if.is_null() {
                crate::compat::HdfObjectManagerFreeObject(dev_mgr_svc_if as *mut crate::types::HdfObject);
                (*inst).devMgrSvcIf = std::ptr::null_mut();
            }
        }
    }
}
