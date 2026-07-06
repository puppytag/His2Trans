//! Module: src_hdf_io_service
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

pub extern "C" fn HdfIoServiceBind(serviceName: *const ::core::ffi::c_char) -> *mut crate::types::HdfIoService {
    unsafe {
        crate::compat::HdfIoServiceAdapterObtain(serviceName)
    }
}

pub extern "C" fn HdfIoServiceRecycle(service: *mut crate::types::HdfIoService) {
    unsafe {
        crate::compat::HdfIoServiceAdapterRecycle(service);
    }
}

pub extern "C" fn HdfIoServicePublish(serviceName: *const ::core::ffi::c_char, mode: u32) -> *mut crate::types::HdfIoService {
    let fp_opt = unsafe { crate::compat::HdfIoServiceAdapterPublish };
    if let Some(fp) = fp_opt {
        unsafe { fp(serviceName, mode) }
    } else {
        ::core::ptr::null_mut()
    }
}

pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    let fp_opt = unsafe { crate::compat::HdfIoServiceAdapterRemove };
    if let Some(fp) = fp_opt {
        unsafe { fp(service); }
    }
}

pub extern "C" fn HdfIoServiceDispatch(ioService: *mut crate::types::HdfIoService, cmdId: ::core::ffi::c_int, data: *mut crate::types::HdfSBuf, reply: *mut crate::types::HdfSBuf) -> i32 {
    if ioService.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let dispatcher = unsafe { (*ioService).dispatcher };
    if dispatcher.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    match unsafe { (*dispatcher).Dispatch } {
        Some(func) => {
            let service_ptr = unsafe { core::ptr::addr_of_mut!((*ioService).object) };
            unsafe { func(service_ptr, cmdId, data, reply) }
        }
        None => crate::types::HDF_ERR_INVALID_OBJECT,
    }
}
