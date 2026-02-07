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
    unsafe { crate::compat::HdfIoServiceAdapterObtain(serviceName) }
}

pub extern "C" fn HdfIoServiceRecycle(service: *mut crate::types::HdfIoService) {
    unsafe {
        crate::compat::HdfIoServiceAdapterRecycle(service);
    }
}

pub extern "C" fn HdfIoServicePublish(serviceName: *const ::core::ffi::c_char, mode: u32) -> *mut crate::types::HdfIoService {
    std::ptr::null_mut()
}

pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        fn HdfIoServiceAdapterRemove(service: *mut crate::types::HdfIoService);
    }
    
    let fn_ptr: Option<unsafe extern "C" fn(*mut crate::types::HdfIoService)> = Some(HdfIoServiceAdapterRemove);
    
    if let Some(f) = fn_ptr {
        unsafe { f(service) };
    }
}

pub extern "C" fn HdfIoServiceDispatch(ioService: *mut crate::types::HdfIoService, cmdId: ::core::ffi::c_int, data: *mut crate::types::HdfSBuf, reply: *mut crate::types::HdfSBuf) -> i32 {
    unsafe {
        if ioService.is_null() {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        
        let dispatcher = (*ioService).dispatcher;
        if dispatcher.is_null() {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        
        let dispatch_fn = (*dispatcher).Dispatch;
        if dispatch_fn.is_none() {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        
        let object_ptr = &mut (*ioService).object as *mut crate::types::HdfObject;
        dispatch_fn.unwrap()(object_ptr, cmdId, data, reply)
    }
}
