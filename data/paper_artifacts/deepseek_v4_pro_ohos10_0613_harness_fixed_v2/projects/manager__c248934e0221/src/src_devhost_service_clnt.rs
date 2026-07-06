//! Module: src_devhost_service_clnt
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

pub extern "C" fn DevHostServiceClntInstallDriver(hostClnt: *mut crate::types::DevHostServiceClnt) -> ::core::ffi::c_int {
    if hostClnt.is_null() {
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        OsalMutexLock(&mut (*hostClnt).hostLock);
    }
    let devHostSvcIf: *mut crate::types::IDevHostService = unsafe { (*hostClnt).hostService };
    if devHostSvcIf.is_null() || unsafe { (*devHostSvcIf).AddDevice.is_none() } {
        unsafe { OsalMutexUnlock(&mut (*hostClnt).hostLock); }
        return crate::types::HDF_FAILURE;
    }
    let add_device_fn = unsafe { (*devHostSvcIf).AddDevice };
    let mut it_curr: *mut crate::types::HdfSListNode = unsafe { (*hostClnt).unloadDevInfos.root };
    while !it_curr.is_null() {
        let node = it_curr;
        it_curr = unsafe { (*node).next };
        let deviceInfo: *mut crate::types::HdfDeviceInfo = node as *mut crate::types::HdfDeviceInfo;
        if deviceInfo.is_null() || (unsafe { (*deviceInfo).preload as u32 }) == crate::types::DEVICE_PRELOAD_DISABLE {
            continue;
        }
        if unsafe { crate::compat::DeviceManagerIsQuickLoad() == crate::types::DEV_MGR_QUICK_LOAD as i32 }
            && (unsafe { (*deviceInfo).preload as u32 }) == crate::types::DEVICE_PRELOAD_ENABLE_STEP2 as u32
        {
            continue;
        }
        if let Some(add_device) = add_device_fn {
            let ret = unsafe { add_device(devHostSvcIf, deviceInfo as *const crate::types::HdfDeviceInfo) };
            if ret != crate::types::HDF_SUCCESS {
                continue;
            }
            unsafe { (*deviceInfo).status = crate::types::HDF_SERVICE_USABLE as u16 };
        } else {
            continue;
        }
    }
    unsafe { OsalMutexUnlock(&mut (*hostClnt).hostLock); }
    crate::types::HDF_SUCCESS
}

// Private unsafe helpers with narrow contracts
unsafe fn init_slists(hostClnt: *mut crate::types::DevHostServiceClnt) {
    HdfSListInit(&mut (*hostClnt).devices as *mut crate::types::HdfSList);
    HdfSListInit(&mut (*hostClnt).unloadDevInfos as *mut crate::types::HdfSList);
    HdfSListInit(&mut (*hostClnt).dynamicDevInfos as *mut crate::types::HdfSList);
}

unsafe fn alloc_and_set_map(hostClnt: *mut crate::types::DevHostServiceClnt) -> *mut crate::types::Map {
    let size = std::mem::size_of::<crate::types::Map>() as crate::types::size_t;
    let map_ptr = OsalMemCalloc(size) as *mut crate::types::Map;
    (*hostClnt).deviceHashMap = map_ptr;
    map_ptr
}

unsafe fn init_mutex(hostClnt: *mut crate::types::DevHostServiceClnt) -> i32 {
    OsalMutexInit(&mut (*hostClnt).hostLock as *mut crate::types::OsalMutex)
}

fn DevHostServiceClntConstruct(hostClnt: *mut crate::types::DevHostServiceClnt)-> i32 {
    unsafe {
        init_slists(hostClnt);
    }

    let map_ptr = unsafe {
        alloc_and_set_map(hostClnt)
    };
    if map_ptr.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"devhost_service_clnt\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s:failed to malloc deviceHashMap\0".as_ptr() as *const ::core::ffi::c_char,
                b"DevHostServiceClntConstruct\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    let mtx_result = unsafe {
        init_mutex(hostClnt)
    };
    if mtx_result != crate::types::HDF_SUCCESS {
        unsafe {
            OsalMemFree(map_ptr as *mut ::core::ffi::c_void);
        }
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        MapInit(map_ptr);
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn DevHostServiceClntNewInstance(hostId: u16, hostName: *const ::core::ffi::c_char) -> *mut crate::types::DevHostServiceClnt {
    let mut host_clnt: *mut crate::types::DevHostServiceClnt = unsafe {
        OsalMemCalloc((::core::mem::size_of::<crate::types::DevHostServiceClnt>() as usize).try_into().unwrap())
            as *mut crate::types::DevHostServiceClnt
    };
    if host_clnt.is_null() {
        return ::core::ptr::null_mut();
    }
    unsafe {
        (*host_clnt).hostId = hostId;
        (*host_clnt).hostName = hostName;
        (*host_clnt).devCount = 0;
        (*host_clnt).hostPid = -1;
        (*host_clnt).stopFlag = false;
    }
    let ret = crate::src_devhost_service_clnt::DevHostServiceClntConstruct(host_clnt);
    if ret != crate::types::HDF_SUCCESS {
        unsafe { OsalMemFree(host_clnt as *mut ::core::ffi::c_void); }
        host_clnt = ::core::ptr::null_mut();
    }
    host_clnt
}

pub extern "C" fn DevHostServiceClntFreeInstance(hostClnt: *mut crate::types::DevHostServiceClnt) {
    if hostClnt.is_null() {
        return;
    }
    unsafe {
        HdfSListFlush(
            &mut (*hostClnt).devices as *mut crate::types::HdfSList,
            Some(crate::src_device_token_clnt::DeviceTokenClntDelete
                as unsafe extern "C" fn(*mut crate::types::HdfSListNode)),
        );
        HdfSListFlush(
            &mut (*hostClnt).unloadDevInfos as *mut crate::types::HdfSList,
            Some(crate::src_devhost_service_clnt::HdfDeviceInfoDelete
                as unsafe extern "C" fn(*mut crate::types::HdfSListNode)),
        );
        HdfSListFlush(
            &mut (*hostClnt).dynamicDevInfos as *mut crate::types::HdfSList,
            Some(crate::src_devhost_service_clnt::HdfDeviceInfoDelete
                as unsafe extern "C" fn(*mut crate::types::HdfSListNode)),
        );
        OsalMemFree((*hostClnt).deviceHashMap as *mut ::core::ffi::c_void);
        let _ = OsalMutexDestroy(&mut (*hostClnt).hostLock as *mut crate::types::OsalMutex);
        OsalMemFree(hostClnt as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn DevHostServiceClntDelete(hostClnt: *mut crate::types::DevHostServiceClnt) {
    if !hostClnt.is_null() {
        unsafe {
            crate::src_devhost_service_clnt::DevHostServiceClntFreeInstance(hostClnt);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn HdfDeviceInfoDelete(listEntry: *mut crate::types::HdfSListNode) {
    if listEntry.is_null() {
        return;
    }
    // HdfSListNode is the first field of HdfDeviceInfo; cast and free the full struct.
    let deviceInfo = listEntry as *mut crate::types::HdfDeviceInfo;
    crate::compat::OsalMemFree(deviceInfo as *mut ::core::ffi::c_void);
}
