//! Module: src_hdf_device_info
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

pub extern "C" fn HdfDeviceInfoConstruct(deviceInfo: *mut crate::types::HdfDeviceInfo) {
    if deviceInfo.is_null() {
        return;
    }
    let info = unsafe { &mut *deviceInfo };
    info.isDynamic = false;
    info.status = crate::types::HDF_SERVICE_UNUSABLE as u16;
    info.deviceType = crate::types::HDF_DEV_LOCAL_SERVICE as u16;
    info.deviceId = 0;
    info.policy = crate::types::SERVICE_POLICY_INVALID as u16;
    info.priority = 0;
    info.preload = crate::types::DEVICE_PRELOAD_ENABLE as u16;
    info.permission = 0;
    info.svcName = std::ptr::null();
    info.moduleName = std::ptr::null();
    info.deviceMatchAttr = std::ptr::null();
    info.deviceName = std::ptr::null();
}

pub extern "C" fn HdfDeviceInfoNewInstance() -> *mut crate::types::HdfDeviceInfo {
    let size = core::mem::size_of::<crate::types::HdfDeviceInfo>();
    let raw = unsafe { crate::compat::OsalMemCalloc(size as crate::types::size_t) };
    let device_info = raw as *mut crate::types::HdfDeviceInfo;
    if !device_info.is_null() {
        crate::src_hdf_device_info::HdfDeviceInfoConstruct(device_info);
        return device_info;
    }
    let tag = b"device_info\0".as_ptr() as *const ::core::ffi::c_char;
    let msg = b"failed to create deviceInfo, oom\0".as_ptr() as *const ::core::ffi::c_char;
    let _ = unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510u32,
            tag,
            msg,
        )
    };
    core::ptr::null_mut()
}

pub extern "C" fn HdfDeviceInfoFreeInstance(deviceInfo: *mut crate::types::HdfDeviceInfo) {
    if !deviceInfo.is_null() {
        let ptr = deviceInfo as *mut ::core::ffi::c_void;
        unsafe {
            OsalMemFree(ptr);
        }
    }
}

pub extern "C" fn HdfDeviceInfoDelete(listEntry: *mut crate::types::HdfSListNode) {
    crate::src_hdf_device_info::HdfDeviceInfoFreeInstance(listEntry as *mut crate::types::HdfDeviceInfo);
}
