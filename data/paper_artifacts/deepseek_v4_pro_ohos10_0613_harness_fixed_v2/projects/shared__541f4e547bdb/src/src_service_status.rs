//! Module: src_service_status
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

pub extern "C" fn ServiceStatusMarshalling(status: *mut crate::types::ServiceStatus, buf: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int {
    if status.is_null() || buf.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let service_name = unsafe { (*status).serviceName };
    if service_name.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let device_class = unsafe { (*status).deviceClass };
    let status_val = unsafe { (*status).status };
    let info = unsafe { (*status).info };
    let empty: u8 = 0;
    let info_ptr = if info.is_null() { &empty as *const u8 as *const i8 } else { info };
    if !unsafe { HdfSbufWriteString(buf, service_name) }
        || !unsafe { HdfSbufWriteUint16(buf, device_class) }
        || !unsafe { HdfSbufWriteUint16(buf, status_val) }
        || !unsafe { HdfSbufWriteString(buf, info_ptr) }
    {
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn ServiceStatusUnMarshalling(status: *mut crate::types::ServiceStatus, buf: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int {
    if status.is_null() || buf.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    let service_name = unsafe { HdfSbufReadString(buf) };
    let sn = service_name as *const ::core::ffi::c_char;
    unsafe {
        (*status).serviceName = sn;
    }
    if service_name.is_null() {
        let tag = b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char;
        let msg = b"failed to unmarshalling service status, service name is null\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xD002510u32,
                tag,
                msg,
            );
        }
        return HDF_FAILURE;
    }
    let ok1 = unsafe { HdfSbufReadUint16(buf, core::ptr::addr_of_mut!((*status).deviceClass) as *mut u16) };
    let ok2 = unsafe { HdfSbufReadUint16(buf, core::ptr::addr_of_mut!((*status).status) as *mut u16) };
    let read_ok = ok1 && ok2;
    if !read_ok {
        let tag = b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char;
        let msg = b"failed to unmarshalling service status, deviceClass or status invalid\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xD002510u32,
                tag,
                msg,
            );
        }
        return HDF_FAILURE;
    }
    let info = unsafe { HdfSbufReadString(buf) };
    let info_ptr = info as *const ::core::ffi::c_char;
    unsafe {
        (*status).info = info_ptr;
    }
    HDF_SUCCESS
}
