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
use std::sync::Once;

static INIT: Once = Once::new();
static mut PM_TASK_QUEUE: *mut crate::types::PmTaskQueue = core::ptr::null_mut();

fn HdfPmTaskQueueInstance()-> *mut crate::types::PmTaskQueue {
    INIT.call_once(|| {
        // Allocate zeroed memory to serve as the singleton PmTaskQueue.
        // The real size is unknown (opaque type), so we use a conservatively
        // large block to avoid breakage.  The original C code used a static
        // local zero‑initialized struct, which this emulates.
        let size: usize = 256;
        let ptr = unsafe { libc::calloc(1, size) as *mut crate::types::PmTaskQueue };
        unsafe { PM_TASK_QUEUE = ptr; }
    });
    // SAFETY: reading a static after initialization; pointer is never null after init.
    unsafe { PM_TASK_QUEUE }
}

pub extern "C" fn HdfPmTaskQueueInit(func: crate::types::HdfTaskFunc) -> *mut crate::types::PmTaskQueue {
    let pm_task_queue = crate::src_hdf_power_manager::HdfPmTaskQueueInstance();
    #[repr(C)]
    struct PmTaskQueueLayout {
        task_queue: *mut ::core::ffi::c_void,
    }
    let internal = pm_task_queue as *mut PmTaskQueueLayout;
    let task_queue_ptr = unsafe { (*internal).task_queue };
    if task_queue_ptr.is_null() {
        let new_queue = unsafe { HdfTaskQueueCreate(func, b"pm_queue\0".as_ptr() as *const ::core::ffi::c_char) as *mut ::core::ffi::c_void };
        unsafe {
            (*internal).task_queue = new_queue;
        }
        if !new_queue.is_null() {
            unsafe {
                HiLogPrint(
                    LOG_CORE,
                    LOG_INFO,
                    0xD002510,
                    b"hdf_power_manager\0".as_ptr() as *const ::core::ffi::c_char,
                    b"%{public}s HdfTaskQueueCreate success\0".as_ptr() as *const ::core::ffi::c_char,
                    b"HdfPmTaskQueueInit\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
    }
    pm_task_queue
}

pub extern "C" fn HdfPmTaskQueueDestroy() {
    let pm_task_queue = unsafe { crate::src_hdf_power_manager::HdfPmTaskQueueInstance() };
    let task_queue = unsafe { *(pm_task_queue as *const *mut crate::types::HdfTaskQueue) };
    unsafe { HdfTaskQueueDestroy(task_queue); }
    unsafe { *(pm_task_queue as *mut *mut crate::types::HdfTaskQueue) = ::core::ptr::null_mut(); }
}

fn PmTaskFunc(para: *mut crate::types::HdfTaskType)-> i32 {
    if para.is_null() {
        return crate::types::HDF_FAILURE;
    }

    // Compute offset of `task` field within HdfPmRequest once
    let offset = {
        let uninit = core::mem::MaybeUninit::<crate::types::HdfPmRequest>::uninit();
        let base = uninit.as_ptr();
        unsafe { core::ptr::addr_of!((*base).task) as usize - base as usize }
    };

    let pm_request = unsafe { (para as *mut u8).sub(offset) as *mut crate::types::HdfPmRequest };
    let (token, pm_type) = unsafe {
        ((*pm_request).token as *mut crate::types::IPowerStateToken, (*pm_request).pmType)
    };

    if pm_type == crate::types::HDF_PM_REQUEST_ACQUIRE {
        if !token.is_null() {
            let token_if = unsafe { &*token };
            if let Some(acquire_fn) = token_if.AcquireWakeLock {
                unsafe { acquire_fn(token) };
            }
        }
    } else if pm_type == crate::types::HDF_PM_REQUEST_RELEASE {
        if !token.is_null() {
            let token_if = unsafe { &*token };
            if let Some(release_fn) = token_if.ReleaseWakeLock {
                unsafe { release_fn(token) };
            }
        }
    }

    unsafe { OsalMemFree(pm_request as *mut ::core::ffi::c_void) };

    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfPmTaskPut(powerToken: *mut crate::types::PowerStateToken, type_: crate::types::HDF_PM_REQUEST_TYPE) {
    if powerToken.is_null() {
        return;
    }

    // Allocate pmRequest
    let pmRequest: *mut crate::types::HdfPmRequest = unsafe {
        let layout = std::alloc::Layout::new::<crate::types::HdfPmRequest>();
        let ptr = std::alloc::alloc(layout);
        if !ptr.is_null() {
            std::ptr::write_bytes(ptr, 0, layout.size());
        }
        ptr as *mut crate::types::HdfPmRequest
    };
    if pmRequest.is_null() {
        return;
    }

    // Transmute PmTaskFunc once to avoid multiple transmute evaluations
    let pm_task_func: unsafe extern "C" fn(*mut crate::types::HdfTaskType) -> i32 = unsafe {
        std::mem::transmute::<
            fn(*mut crate::types::HdfTaskType) -> i32,
            unsafe extern "C" fn(*mut crate::types::HdfTaskType) -> i32,
        >(crate::src_hdf_power_manager::PmTaskFunc)
    };

    // Assign fields (raw pointer writes)
    unsafe {
        (*pmRequest).token = powerToken;
        (*pmRequest).pmType = type_;
    }
    unsafe {
        (*pmRequest).task.func = Some(pm_task_func);
    }

    // Enqueue task
    unsafe {
        let pmTaskQueue = crate::src_hdf_power_manager::HdfPmTaskQueueInstance();
        let task_queue = *(pmTaskQueue as *const *mut crate::types::HdfTaskQueue);
        HdfTaskEnqueue(task_queue, &mut (*pmRequest).task as *mut crate::types::HdfTaskType);
    }
}

pub extern "C" fn HdfPowerManagerInit() -> i32 {
    unsafe {
        DevMgrPmRegister();
    }
    crate::src_hdf_power_manager::HdfPmTaskQueueInit(None);
    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfPowerManagerExit() {
    crate::src_hdf_power_manager::HdfPmTaskQueueDestroy();
}
