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

fn on_io_service_event_receive_safe(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    if listener.is_null() || service.is_null() || data.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let _ = id;
    let mut status: crate::types::ServiceStatus = unsafe { core::mem::zeroed() };
    let ret = crate::src_service_status::ServiceStatusUnMarshalling(
        &mut status as *mut crate::types::ServiceStatus,
        data,
    );
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let priv_ = unsafe { (*listener).priv_ };
    let status_listener = priv_ as *mut crate::types::IoServiceStatusListener;
    let cb = unsafe { (*status_listener).svcstatListener.callback };
    if let Some(callback) = cb {
        let mask = unsafe { (*status_listener).deviceClass };
        if (mask & status.deviceClass) != 0 {
            let svc_ptr = unsafe { core::ptr::addr_of_mut!((*status_listener).svcstatListener) };
            let status_ptr = &mut status as *mut crate::types::ServiceStatus;
            unsafe {
                callback(svc_ptr, status_ptr);
            }
        }
    }
    crate::types::HDF_SUCCESS
}

unsafe extern "C" fn OnIoServiceEventReceive(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    on_io_service_event_receive_safe(listener, service, id, data)
}

pub extern "C" fn IoServiceStatusListenerNewInstance() -> *mut crate::types::ServiceStatusListener {
    let size = core::mem::size_of::<IoServiceStatusListener>();
    let raw = unsafe { crate::compat::OsalMemCalloc(size as crate::types::size_t) };
    let listener = raw as *mut IoServiceStatusListener;
    if listener.is_null() {
        return core::ptr::null_mut();
    }
    let priv_ptr = listener as *mut core::ffi::c_void;
    unsafe {
        (*listener).ioservListener.onReceive = Some(crate::src_ioserstat_listener::OnIoServiceEventReceive);
        (*listener).ioservListener.priv_ = priv_ptr;
        core::ptr::addr_of_mut!((*listener).svcstatListener)
    }
}

pub extern "C" fn IoServiceStatusListenerFree(listener: *mut crate::types::ServiceStatusListener) {
    if listener.is_null() {
        return;
    }
    let ioserv_listener = listener as *mut crate::types::IoServiceStatusListener;
    let ptr = ioserv_listener as *mut ::core::ffi::c_void;
    unsafe {
        OsalMemFree(ptr);
    }
}
