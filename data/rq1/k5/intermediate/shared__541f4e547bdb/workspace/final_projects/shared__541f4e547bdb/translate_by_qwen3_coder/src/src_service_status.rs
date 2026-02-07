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
    let status_ref = unsafe { &*status };
    if status_ref.serviceName.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let info_ptr = if !status_ref.info.is_null() {
        status_ref.info
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    };
    unsafe {
        if !crate::compat::HdfSbufWriteString(buf, status_ref.serviceName)
            || !crate::compat::HdfSbufWriteUint16(buf, status_ref.deviceClass)
            || !crate::compat::HdfSbufWriteUint16(buf, status_ref.status)
            || !crate::compat::HdfSbufWriteString(buf, info_ptr)
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to marshalling service status\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return crate::types::HDF_FAILURE;
        }
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn ServiceStatusUnMarshalling(status: *mut crate::types::ServiceStatus, buf: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int {
    if status.is_null() || buf.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        (*status).serviceName = crate::compat::HdfSbufReadString(buf);
        if (*status).serviceName.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to unmarshalling service status, service name is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return crate::types::HDF_FAILURE;
        }
        if !crate::compat::HdfSbufReadUint16(buf, &mut (*status).deviceClass)
            || !crate::compat::HdfSbufReadUint16(buf, &mut (*status).status)
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to unmarshalling service status, deviceClass or status invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return crate::types::HDF_FAILURE;
        }
        (*status).info = crate::compat::HdfSbufReadString(buf);
    }
    crate::types::HDF_SUCCESS
}
