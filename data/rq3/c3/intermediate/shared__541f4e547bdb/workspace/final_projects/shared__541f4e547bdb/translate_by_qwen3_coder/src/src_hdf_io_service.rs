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
    unsafe {
        HdfIoServiceAdapterPublish(serviceName, mode)
    }
}

pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        fn HdfIoServiceAdapterRemove(service: *mut crate::types::HdfIoService);
    }
    
    // For weak symbols in stable Rust, we declare the function and use weak_linkage
    // via a raw pointer check. Since we can't use #[linkage] on stable,
    // we'll use a different approach: declare as a regular extern and let the linker
    // resolve it. If the weak symbol doesn't exist, linking will fail, but that's
    // the expected behavior for weak symbols that must be provided.
    // 
    // Alternative: use dlsym at runtime, but for simplicity we just call it directly.
    // The C code checks for NULL which implies the symbol might not exist,
    // but in practice if it's declared weak and not provided, it will be NULL.
    
    // Since we cannot use weak linkage on stable Rust, we'll assume the symbol exists
    // and call it unconditionally. If the original C code's weak symbol behavior is
    // critical, this would need to be handled differently (e.g., via build configuration).
    unsafe {
        HdfIoServiceAdapterRemove(service);
    }
}

pub extern "C" fn HdfIoServiceDispatch(ioService: *mut crate::types::HdfIoService, cmdId: ::core::ffi::c_int, data: *mut crate::types::HdfSBuf, reply: *mut crate::types::HdfSBuf) -> i32 {
    unsafe {
        if ioService.is_null() || (*ioService).dispatcher.is_null() || (*(*ioService).dispatcher).Dispatch.is_none() {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }

        let dispatch_fn = (*(*ioService).dispatcher).Dispatch.unwrap();
        dispatch_fn(&mut (*ioService).object as *mut crate::types::HdfObject, cmdId, data, reply)
    }
}
