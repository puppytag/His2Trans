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
    // HdfIoServiceAdapterPublish is not available in globals or compat
    // The C code checks if it's non-NULL and calls it, otherwise returns NULL
    // Since the symbol is not available, we just return null
    std::ptr::null_mut()
}

pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        fn HdfIoServiceAdapterRemove(service: *mut crate::types::HdfIoService);
    }
    
    // For weak symbols, we need to check if the symbol is defined
    // In C, weak symbols can be NULL if not defined
    // We'll just call it directly since Rust doesn't have a direct way to check weak symbols
    // The linker will resolve this - if the weak symbol exists it will be called
    unsafe {
        let func_ptr: *const () = HdfIoServiceAdapterRemove as *const ();
        if !func_ptr.is_null() {
            HdfIoServiceAdapterRemove(service);
        }
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
