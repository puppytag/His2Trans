//! Module: src_ioserstat_listener
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

fn OnIoServiceEventReceive(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    if listener.is_null() || service.is_null() || data.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let _ = id;
    
    let mut status = crate::types::ServiceStatus {
        serviceName: std::ptr::null(),
        deviceClass: 0,
        status: 0,
        info: std::ptr::null(),
    };
    
    if crate::src_service_status::ServiceStatusUnMarshalling(&mut status as *mut crate::types::ServiceStatus, data) != crate::types::HDF_SUCCESS {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    
    unsafe {
        let statusListener = (*listener).priv_ as *mut crate::types::IoServiceStatusListener;
        if (*statusListener).svcstatListener.callback.is_some() &&
           ((*statusListener).deviceClass & status.deviceClass) != 0 {
            if let Some(callback) = (*statusListener).svcstatListener.callback {
                callback(
                    &mut (*statusListener).svcstatListener as *mut crate::types::ServiceStatusListener,
                    &mut status as *mut crate::types::ServiceStatus
                );
            }
        }
    }
    
    crate::types::HDF_SUCCESS
}

pub extern "C" fn IoServiceStatusListenerNewInstance() -> *mut crate::types::ServiceStatusListener {
    let listener = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::IoServiceStatusListener>() as u32)
    } as *mut crate::types::IoServiceStatusListener;
    
    if listener.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        (*listener).ioservListener.onReceive = Some(std::mem::transmute::<
            fn(*mut crate::types::HdfDevEventlistener, *mut crate::types::HdfIoService, u32, *mut crate::types::HdfSBuf) -> i32,
            unsafe extern "C" fn(*mut crate::types::HdfDevEventlistener, *mut crate::types::HdfIoService, u32, *mut crate::types::HdfSBuf) -> ::core::ffi::c_int
        >(OnIoServiceEventReceive));
        (*listener).ioservListener.priv_ = listener as *mut ::core::ffi::c_void;
        
        &mut (*listener).svcstatListener as *mut crate::types::ServiceStatusListener
    }
}

pub extern "C" fn IoServiceStatusListenerFree(listener: *mut crate::types::ServiceStatusListener) {
    if listener.is_null() {
        return;
    }
    
    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    // Calculate offset of svcstatListener field in IoServiceStatusListener
    let offset = std::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener);
    let ioservListener = unsafe {
        (listener as *mut u8).offset(-(offset as isize)) as *mut crate::types::IoServiceStatusListener
    };
    
    unsafe {
        crate::compat::OsalMemFree(ioservListener as *mut ::core::ffi::c_void);
    }
}
