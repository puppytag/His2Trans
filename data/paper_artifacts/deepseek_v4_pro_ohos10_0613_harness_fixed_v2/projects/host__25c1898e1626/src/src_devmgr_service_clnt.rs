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
    let inst = crate::src_devmgr_service_clnt::DevmgrServiceClntGetInstance();
    if inst.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dev_mgr_svc_if = unsafe { (*inst).devMgrSvcIf };
    if dev_mgr_svc_if.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let attach_fn = unsafe {
        match (*dev_mgr_svc_if).AttachDeviceHost {
            Some(f) => f,
            None => return crate::types::HDF_FAILURE,
        }
    };
    unsafe { attach_fn(dev_mgr_svc_if, hostId, hostService) }
}

pub extern "C" fn DevmgrServiceClntAttachDevice(deviceToken: *mut crate::types::IHdfDeviceToken) -> ::core::ffi::c_int {
    let inst = crate::src_devmgr_service_clnt::DevmgrServiceClntGetInstance();
    if inst.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let dev_mgr_svc_if = unsafe { (*inst).devMgrSvcIf };
    if dev_mgr_svc_if.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let attach_fn = unsafe { (*dev_mgr_svc_if).AttachDevice };
    if attach_fn.is_none() {
        return crate::types::HDF_FAILURE;
    }
    unsafe { attach_fn.unwrap()(dev_mgr_svc_if, deviceToken) }
}

pub extern "C" fn DevmgrServiceClntDetachDevice(devid: crate::types::devid_t) -> ::core::ffi::c_int {
    let inst = crate::src_devmgr_service_clnt::DevmgrServiceClntGetInstance();
    if inst.is_null() || unsafe { (*inst).devMgrSvcIf.is_null() } {
        return crate::types::HDF_FAILURE;
    }
    let dev_mgr_svc_if = unsafe { (*inst).devMgrSvcIf };
    let detach = unsafe { (*dev_mgr_svc_if).DetachDevice };
    match detach {
        Some(f) => unsafe { f(dev_mgr_svc_if, devid) },
        None => crate::types::HDF_FAILURE,
    }
}

pub extern "C" fn DevmgrServiceClntGetInstance() -> *mut crate::types::DevmgrServiceClnt {
    use core::sync::atomic::{AtomicBool, Ordering};
    static INIT: AtomicBool = AtomicBool::new(false);
    static mut INSTANCE: crate::types::DevmgrServiceClnt = crate::types::DevmgrServiceClnt {
        devMgrSvcIf: std::ptr::null_mut(),
    };
    if !INIT.load(Ordering::Acquire) {
        let obj = unsafe {
            HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVMGR_SERVICE.try_into().unwrap())
                as *mut crate::types::IDevmgrService
        };
        unsafe { INSTANCE.devMgrSvcIf = obj; }
        INIT.store(true, Ordering::Release);
    }
    unsafe { core::ptr::addr_of_mut!(INSTANCE) }
}

pub extern "C" fn DevmgrServiceClntFreeInstance(inst: *mut crate::types::DevmgrServiceClnt) {
    if inst.is_null() {
        return;
    }
    unsafe {
        let dev_mgr_svc_if = (*inst).devMgrSvcIf;
        if !dev_mgr_svc_if.is_null() {
            HdfObjectManagerFreeObject(dev_mgr_svc_if as *mut crate::types::HdfObject);
            (*inst).devMgrSvcIf = std::ptr::null_mut();
        }
    }
}
