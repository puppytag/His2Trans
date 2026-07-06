//! Module: src_hdf_service_subscriber
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

pub extern "C" fn HdfServiceSubscriberObtain(callback: crate::types::SubscriberCallback, devid: crate::types::devid_t) -> *mut crate::types::HdfServiceSubscriber {
    let size = std::mem::size_of::<crate::types::HdfServiceSubscriber>();
    let ptr = unsafe { libc::calloc(1, size as libc::size_t) as *mut crate::types::HdfServiceSubscriber };
    if !ptr.is_null() {
        unsafe {
            (*ptr).state = crate::types::HDF_SUBSCRIBER_STATE_PENDING;
            (*ptr).devId = devid;
            (*ptr).callback = callback;
        }
    }
    ptr
}

pub extern "C" fn HdfServiceSubscriberRecycle(subscriber: *mut crate::types::HdfServiceSubscriber) {
    if !subscriber.is_null() {
        unsafe {
            OsalMemFree(subscriber as *mut ::core::ffi::c_void);
        }
    }
}

pub extern "C" fn HdfServiceSubscriberDelete(listEntry: *mut crate::types::HdfSListNode) {
    let subscriber = listEntry as *mut crate::types::HdfServiceSubscriber;
    unsafe {
        if !subscriber.is_null() {
            crate::src_hdf_service_subscriber::HdfServiceSubscriberRecycle(subscriber);
        }
    }
}
