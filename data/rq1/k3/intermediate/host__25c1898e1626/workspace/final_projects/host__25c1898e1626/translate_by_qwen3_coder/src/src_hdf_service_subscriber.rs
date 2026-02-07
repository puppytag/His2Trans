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
    let service_subscriber = unsafe { libc::calloc(1, std::mem::size_of::<crate::types::HdfServiceSubscriber>()) } as *mut crate::types::HdfServiceSubscriber;
    if !service_subscriber.is_null() {
        unsafe {
            (*service_subscriber).state = crate::types::HDF_SUBSCRIBER_STATE_PENDING as u32;
            (*service_subscriber).devId = devid;
            (*service_subscriber).callback = callback;
        }
    }
    service_subscriber
}

pub extern "C" fn HdfServiceSubscriberRecycle(subscriber: *mut crate::types::HdfServiceSubscriber) {
    if !subscriber.is_null() {
        unsafe {
            crate::compat::OsalMemFree(subscriber as *mut ::core::ffi::c_void);
        }
    }
}

pub extern "C" fn HdfServiceSubscriberDelete(listEntry: *mut crate::types::HdfSListNode) {
    let serviceSubscriber = listEntry as *mut crate::types::HdfServiceSubscriber;
    if !serviceSubscriber.is_null() {
        crate::src_hdf_service_subscriber::HdfServiceSubscriberRecycle(serviceSubscriber);
    }
}
