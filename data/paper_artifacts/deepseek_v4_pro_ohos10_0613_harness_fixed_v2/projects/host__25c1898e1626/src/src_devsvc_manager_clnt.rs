//! Module: src_devsvc_manager_clnt
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

pub extern "C" fn DevSvcManagerClntAddService(service: *mut crate::types::HdfDeviceObject, servinfo: *const crate::types::HdfServiceInfo) -> ::core::ffi::c_int {
    let dev_svc_mgr_clnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if dev_svc_mgr_clnt.is_null() {
        return crate::types::HDF_FAILURE;
    }
    if unsafe { (*servinfo).devClass as crate::types::DeviceClass } >= crate::types::DEVICE_CLASS_MAX {
        return crate::types::HDF_FAILURE;
    }
    let service_manager = unsafe { (*dev_svc_mgr_clnt).devSvcMgrIf };
    if service_manager.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let add_service = unsafe { (*service_manager).AddService };
    if let Some(func) = add_service {
        unsafe { func(service_manager, service, servinfo) }
    } else {
        crate::types::HDF_FAILURE
    }
}

pub extern "C" fn DevSvcManagerClntUpdateService(service: *mut crate::types::HdfDeviceObject, servinfo: *const crate::types::HdfServiceInfo) -> ::core::ffi::c_int {
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if devSvcMgrClnt.is_null() {
        // logging omitted (HiLogPrint unavailable in Rust context)
        return crate::types::HDF_FAILURE;
    }
    let dev_class = unsafe { (*servinfo).devClass };
    if (dev_class as u32) >= crate::types::DEVICE_CLASS_MAX {
        // logging omitted
        return crate::types::HDF_FAILURE;
    }
    let serviceManager = unsafe { (*devSvcMgrClnt).devSvcMgrIf };
    if serviceManager.is_null() || unsafe { (*serviceManager).UpdateService }.is_none() {
        // logging omitted
        return crate::types::HDF_FAILURE;
    }
    let update_fn = unsafe { (*serviceManager).UpdateService.unwrap() };
    unsafe { update_fn(serviceManager, service, servinfo) }
}

pub extern "C" fn DevSvcManagerClntGetService(svcName: *const ::core::ffi::c_char) -> *const crate::types::HdfObject {
    let dev_svc_mgr_clnt = unsafe { crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance() };
    if dev_svc_mgr_clnt.is_null() {
        return std::ptr::null();
    }
    let service_manager = unsafe { (*dev_svc_mgr_clnt).devSvcMgrIf };
    if service_manager.is_null() {
        return std::ptr::null();
    }
    let get_service_fn = unsafe { (*service_manager).GetService };
    match get_service_fn {
        Some(func) => unsafe { func(service_manager, svcName) as *const crate::types::HdfObject },
        None => std::ptr::null(),
    }
}

pub extern "C" fn DevSvcManagerClntGetDeviceObject(svcName: *const ::core::ffi::c_char) -> *mut crate::types::HdfDeviceObject {
    let dev_svc_mgr_clnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if dev_svc_mgr_clnt.is_null() {
        return ::core::ptr::null_mut();
    }
    let service_manager = unsafe { (*dev_svc_mgr_clnt).devSvcMgrIf };
    if service_manager.is_null() {
        return ::core::ptr::null_mut();
    }
    let get_object = unsafe { (*service_manager).GetObject };
    if let Some(get_object_fn) = get_object {
        unsafe { get_object_fn(service_manager, svcName) }
    } else {
        ::core::ptr::null_mut()
    }
}

pub extern "C" fn DevSvcManagerClntSubscribeService(svcName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> ::core::ffi::c_int {
    let dev_svc_mgr_clnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if dev_svc_mgr_clnt.is_null() || svcName.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let service_manager = unsafe { (*dev_svc_mgr_clnt).devSvcMgrIf };
    if service_manager.is_null() {
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        match (*service_manager).SubscribeService {
            Some(subscribe_fn) => subscribe_fn(service_manager, svcName, callback),
            None => crate::types::HDF_FAILURE,
        }
    }
}

pub extern "C" fn DevSvcManagerClntUnsubscribeService(svcName: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let dev_svc_mgr_clnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if dev_svc_mgr_clnt.is_null() || svcName.is_null() {
        return crate::types::HDF_FAILURE;
    }
    let service_manager = unsafe { (*dev_svc_mgr_clnt).devSvcMgrIf };
    if service_manager.is_null() || unsafe { (*service_manager).UnsubscribeService.is_none() } {
        return crate::types::HDF_FAILURE;
    }
    unsafe { (*service_manager).UnsubscribeService.unwrap()(service_manager, svcName) }
}

pub extern "C" fn DevSvcManagerClntRemoveService(svcName: *const ::core::ffi::c_char) {
    let devSvcMgrClnt = unsafe { crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance() };
    if devSvcMgrClnt.is_null() {
        return;
    }
    let serviceManager = unsafe { (*devSvcMgrClnt).devSvcMgrIf };
    if serviceManager.is_null() {
        return;
    }
    let removeService = unsafe { (*serviceManager).RemoveService };
    if let Some(func) = removeService {
        unsafe { func(serviceManager, svcName, ::std::ptr::null()) };
    }
}

fn DevSvcManagerClntConstruct(inst: *mut crate::types::DevSvcManagerClnt) {
    unsafe {
        (*inst).devSvcMgrIf = HdfObjectManagerGetObject((crate::types::HDF_OBJECT_ID_DEVSVC_MANAGER as u32).try_into().unwrap())
                              as *mut crate::types::IDevSvcManager;
    }
}

pub extern "C" fn DevSvcManagerClntGetInstance() -> *mut crate::types::DevSvcManagerClnt {
    use core::sync::atomic::{AtomicBool, Ordering};
    static INIT: AtomicBool = AtomicBool::new(false);
    static mut SINGLETON: core::mem::MaybeUninit<crate::types::DevSvcManagerClnt> = core::mem::MaybeUninit::uninit();
    static mut INSTANCE_PTR: *mut crate::types::DevSvcManagerClnt = core::ptr::null_mut();
    if !INIT.load(Ordering::Acquire) {
        let ptr = unsafe { SINGLETON.as_mut_ptr() };
        crate::src_devsvc_manager_clnt::DevSvcManagerClntConstruct(ptr);
        unsafe { INSTANCE_PTR = ptr; }
        INIT.store(true, Ordering::Release);
    }
    unsafe { INSTANCE_PTR }
}

pub extern "C" fn DevSvcManagerClntFreeInstance(instance: *mut crate::types::DevSvcManagerClnt) {
    if !instance.is_null() {
        unsafe {
            let dev_svc_mgr_if = (*instance).devSvcMgrIf;
            HdfObjectManagerFreeObject(dev_svc_mgr_if as *mut crate::types::HdfObject);
        }
    }
}
