//! Module: src_hdf_power_manager
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

fn HdfPmTaskQueueInstance() -> *mut crate::types::PmTaskQueue {
    static mut PM_TASK_QUEUE: crate::types::PmTaskQueue = unsafe { ::core::mem::MaybeUninit::zeroed().assume_init() };
    unsafe { &mut PM_TASK_QUEUE }
}

pub extern "C" fn HdfPmTaskQueueInit(func: crate::types::HdfTaskFunc) -> *mut crate::types::PmTaskQueue {
    let pmTaskQueue = crate::src_hdf_power_manager::HdfPmTaskQueueInstance();
    if pmTaskQueue.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        if (*pmTaskQueue).taskQueue.is_null() {
            (*pmTaskQueue).taskQueue = HdfTaskQueueCreate(func, b"pm_queue\0".as_ptr() as *const _);
            if !(*pmTaskQueue).taskQueue.is_null() {
                let _ = HiLogPrint(
                    LOG_CORE,
                    LOG_INFO,
                    0xD002510,
                    b"hdf_power_manager\0".as_ptr() as *const _,
                    b"%{public}s HdfTaskQueueCreate success\0".as_ptr() as *const _,
                    b"HdfPmTaskQueueInit\0".as_ptr() as *const _,
                );
            }
        }
    }
    pmTaskQueue
}

pub extern "C" fn HdfPmTaskQueueDestroy() {
    let pmTaskQueue = crate::src_hdf_power_manager::HdfPmTaskQueueInstance();
    if !pmTaskQueue.is_null() {
        unsafe {
            let taskQueue = (*pmTaskQueue).taskQueue;
            if !taskQueue.is_null() {
                HdfTaskQueueDestroy(taskQueue);
                (*pmTaskQueue).taskQueue = std::ptr::null_mut();
            }
        }
    }
}

fn PmTaskFunc(para: *mut crate::types::HdfTaskType) -> i32 {
    use crate::types::*;
    if para.is_null() {
        return HDF_FAILURE;
    }
    let pm_request_ptr = unsafe {
        (para as *mut u8).offset(-(std::mem::offset_of!(HdfPmRequest, task) as isize))
            as *mut HdfPmRequest
    };
    let pm_request = unsafe { &*pm_request_ptr };
    let token_if = pm_request.token as *mut IPowerStateToken;
    if pm_request.pmType == HDF_PM_REQUEST_ACQUIRE {
        if !token_if.is_null() {
            let token_if_ref = unsafe { &*token_if };
            if let Some(func) = token_if_ref.AcquireWakeLock {
                unsafe { func(token_if) };
            }
        }
    } else if pm_request.pmType == HDF_PM_REQUEST_RELEASE {
        if !token_if.is_null() {
            let token_if_ref = unsafe { &*token_if };
            if let Some(func) = token_if_ref.ReleaseWakeLock {
                unsafe { func(token_if) };
            }
        }
    }
    unsafe { crate::compat::OsalMemFree(pm_request_ptr as *mut core::ffi::c_void) };
    HDF_SUCCESS
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_hdf_power_manager_5
// c_function: HdfPmTaskPut
// rust_file: src_hdf_power_manager.rs
// rust_signature: pub extern "C" fn HdfPmTaskPut(powerToken: *mut crate::types::PowerStateToken, type_: crate::types::HDF_PM_REQUEST_TYPE)
// c_first_line: void HdfPmTaskPut(struct PowerStateToken *powerToken, HDF_PM_REQUEST_TYPE type)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk1-again/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_power_manager_5/translated_rust.rs
// last_error_truncated:
//   error[E0605]: non-primitive cast: `fn(*mut HdfTaskType) -> i32 {PmTaskFunc}` as `unsafe extern "C" fn(*mut HdfTaskType) -> i32`
//      --> src/src_hdf_power_manager.rs:101:39
//       |
//       |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid cast
//   error: could not compile `host__25c1898e1626` (bin "host__25c1898e1626") due to 1 previous error; 24 warnings emitted
// =================================
pub extern "C" fn HdfPmTaskPut(powerToken: *mut crate::types::PowerStateToken, type_: crate::types::HDF_PM_REQUEST_TYPE) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_hdf_power_manager::HdfPmTaskPut(powerToken as _, type_ as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_hdf_power_manager_5
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk1-again/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_power_manager_5/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfPmTaskPut(powerToken: *mut crate::types::PowerStateToken, type_: crate::types::HDF_PM_REQUEST_TYPE) {
    use crate::compat::*;
    use crate::globals::*;
    if powerToken.is_null() {
        return;
    }
    let pmTaskQueue = crate::src_hdf_power_manager::HdfPmTaskQueueInstance();
    let pmRequest = unsafe { libc::calloc(1, std::mem::size_of::<crate::types::HdfPmRequest>()) as *mut crate::types::HdfPmRequest };
    if pmRequest.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD002510, b"hdf_power_manager\0".as_ptr() as *const _, b"%{public}s OsalMemCalloc fail\0".as_ptr() as *const _, b"HdfPmTaskPut\0".as_ptr() as *const _);
        return;
    }
    unsafe {
        (*pmRequest).token = powerToken;
        (*pmRequest).pmType = type_;
        (*pmRequest).task.func = Some(crate::src_hdf_power_manager::PmTaskFunc as unsafe extern "C" fn(*mut crate::types::HdfTaskType) -> i32);
        let task_queue = (*pmTaskQueue).taskQueue;
        HdfTaskEnqueue(task_queue, &mut (*pmRequest).task);
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_hdf_power_manager_5
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn HdfPowerManagerInit() -> i32 {
    unsafe {
        DevMgrPmRegister();
        crate::src_hdf_power_manager::HdfPmTaskQueueInit(None);
        crate::types::HDF_SUCCESS
    }
}

pub extern "C" fn HdfPowerManagerExit() {
    crate::src_hdf_power_manager::HdfPmTaskQueueDestroy();
}
