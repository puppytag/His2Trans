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
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if devSvcMgrClnt.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devsvc_manager_clnt\0".as_ptr() as *const _, b"failed to add service, client is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    if unsafe { (*servinfo).devClass } >= crate::types::DEVICE_CLASS_MAX as u16 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devsvc_manager_clnt\0".as_ptr() as *const _, b"failed to add service, invalid class\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    let serviceManager = unsafe { (*devSvcMgrClnt).devSvcMgrIf };
    if serviceManager.is_null() || unsafe { (*serviceManager).AddService }.is_none() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devsvc_manager_clnt\0".as_ptr() as *const _, b"serviceManager AddService function is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    if let Some(f) = unsafe { (*serviceManager).AddService } {
        unsafe { f(serviceManager, service, servinfo) }
    } else {
        crate::types::HDF_FAILURE
    }
}

pub extern "C" fn DevSvcManagerClntUpdateService(service: *mut crate::types::HdfDeviceObject, servinfo: *const crate::types::HdfServiceInfo) -> ::core::ffi::c_int {
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if devSvcMgrClnt.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devsvc_manager_clnt\0".as_ptr() as *const _, b"failed to update service, client is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    if unsafe { (*servinfo).devClass } >= crate::types::DEVICE_CLASS_MAX as u16 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devsvc_manager_clnt\0".as_ptr() as *const _, b"failed to update service, invalid class\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    let serviceManager = unsafe { (*devSvcMgrClnt).devSvcMgrIf };
    if serviceManager.is_null() || unsafe { (*serviceManager).UpdateService }.is_none() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devsvc_manager_clnt\0".as_ptr() as *const _, b"serviceManager UpdateService function is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    if let Some(f) = unsafe { (*serviceManager).UpdateService } {
        unsafe { f(serviceManager, service, servinfo) }
    } else {
        crate::types::HDF_FAILURE
    }
}

pub extern "C" fn DevSvcManagerClntGetService(svcName: *const ::core::ffi::c_char) -> *const crate::types::HdfObject {
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    let mut serviceManager: *mut crate::types::IDevSvcManager = std::ptr::null_mut();
    if devSvcMgrClnt.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"devsvc_manager_clnt\0".as_ptr() as *const _,
            b"failed to get service, client is null\0".as_ptr() as *const _,
        ) };
        return std::ptr::null();
    }
    unsafe {
        serviceManager = (*devSvcMgrClnt).devSvcMgrIf;
    }
    if serviceManager.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"devsvc_manager_clnt\0".as_ptr() as *const _,
            b"serviceManager GetService function is null\0".as_ptr() as *const _,
        ) };
        return std::ptr::null();
    }
    let get_service = unsafe { (*serviceManager).GetService };
    if let Some(f) = get_service {
        unsafe { f(serviceManager, svcName) as *const crate::types::HdfObject }
    } else {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"devsvc_manager_clnt\0".as_ptr() as *const _,
            b"serviceManager GetService function is null\0".as_ptr() as *const _,
        ) };
        std::ptr::null()
    }
}

pub extern "C" fn DevSvcManagerClntGetDeviceObject(svcName: *const ::core::ffi::c_char) -> *mut crate::types::HdfDeviceObject {
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if devSvcMgrClnt.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            "devsvc_manager_clnt\0".as_ptr() as *const _,
            "failed to get device object, client is null\0".as_ptr() as *const _,
        ) };
        return std::ptr::null_mut();
    }
    let serviceManager = unsafe { (*devSvcMgrClnt).devSvcMgrIf };
    if serviceManager.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            "devsvc_manager_clnt\0".as_ptr() as *const _,
            "failed to get device object, method not implement\0".as_ptr() as *const _,
        ) };
        return std::ptr::null_mut();
    }
    let get_object = unsafe { (*serviceManager).GetObject };
    if let Some(f) = get_object {
        unsafe { f(serviceManager, svcName) }
    } else {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            "devsvc_manager_clnt\0".as_ptr() as *const _,
            "failed to get device object, method not implement\0".as_ptr() as *const _,
        ) };
        std::ptr::null_mut()
    }
}

pub extern "C" fn DevSvcManagerClntSubscribeService(svcName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> ::core::ffi::c_int {
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    let mut serviceManager: *mut crate::types::IDevSvcManager = std::ptr::null_mut();
    if devSvcMgrClnt.is_null() || svcName.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devsvc_manager_clnt\0".as_ptr() as *const _, b"failed to subscribe service, client or svcName is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        serviceManager = (*devSvcMgrClnt).devSvcMgrIf;
    }
    if serviceManager.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devsvc_manager_clnt\0".as_ptr() as *const _, b"failed to subscribe service, method not implement\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        if let Some(f) = (*serviceManager).SubscribeService {
            f(serviceManager, svcName, callback)
        } else {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devsvc_manager_clnt\0".as_ptr() as *const _, b"failed to subscribe service, method not implement\0".as_ptr() as *const _);
            crate::types::HDF_FAILURE
        }
    }
}

pub extern "C" fn DevSvcManagerClntUnsubscribeService(svcName: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    let mut serviceManager: *mut crate::types::IDevSvcManager = std::ptr::null_mut();
    if devSvcMgrClnt.is_null() || svcName.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"devsvc_manager_clnt\0".as_ptr() as *const _,
            b"failed to unsubscribe service, client or svcName is null\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        serviceManager = (*devSvcMgrClnt).devSvcMgrIf;
    }
    if serviceManager.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"devsvc_manager_clnt\0".as_ptr() as *const _,
            b"failed to unsubscribe service, method not implement\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_FAILURE;
    }
    let unsubscribe_func = unsafe { (*serviceManager).UnsubscribeService };
    if let Some(func) = unsubscribe_func {
        unsafe { func(serviceManager, svcName) }
    } else {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"devsvc_manager_clnt\0".as_ptr() as *const _,
            b"failed to unsubscribe service, method not implement\0".as_ptr() as *const _,
        ) };
        crate::types::HDF_FAILURE
    }
}

pub extern "C" fn DevSvcManagerClntRemoveService(svcName: *const ::core::ffi::c_char) {
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if devSvcMgrClnt.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(3, 6, 0xD002510, b"devsvc_manager_clnt\0".as_ptr() as *const _, b"failed to remove service, devSvcMgrClnt is null\0".as_ptr() as *const _) };
        return;
    }
    let serviceManager = unsafe { (*devSvcMgrClnt).devSvcMgrIf };
    if serviceManager.is_null() {
        return;
    }
    let remove_service = unsafe { (*serviceManager).RemoveService };
    if let Some(f) = remove_service {
        unsafe { f(serviceManager, svcName, std::ptr::null()) };
    }
}

fn DevSvcManagerClntConstruct(inst: *mut crate::types::DevSvcManagerClnt) {
    unsafe {
        (*inst).devSvcMgrIf = crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVSVC_MANAGER as i32) as *mut crate::types::IDevSvcManager;
    }
}

pub extern "C" fn DevSvcManagerClntGetInstance() -> *mut crate::types::DevSvcManagerClnt {
    static mut INSTANCE: *mut crate::types::DevSvcManagerClnt = std::ptr::null_mut();
    unsafe {
        if INSTANCE.is_null() {
            static mut SINGLETON_INSTANCE: crate::types::DevSvcManagerClnt = crate::types::DevSvcManagerClnt {
                devSvcMgrIf: std::ptr::null_mut(),
            };
            crate::src_devsvc_manager_clnt::DevSvcManagerClntConstruct(&mut SINGLETON_INSTANCE);
            INSTANCE = &mut SINGLETON_INSTANCE;
        }
        INSTANCE
    }
}

pub extern "C" fn DevSvcManagerClntFreeInstance(instance: *mut crate::types::DevSvcManagerClnt) {
    if !instance.is_null() {
        unsafe {
            let dev_svc_mgr_if = (*instance).devSvcMgrIf;
            if !dev_svc_mgr_if.is_null() {
                crate::compat::HdfObjectManagerFreeObject(dev_svc_mgr_if as *mut crate::types::HdfObject);
            }
        }
    }
}
