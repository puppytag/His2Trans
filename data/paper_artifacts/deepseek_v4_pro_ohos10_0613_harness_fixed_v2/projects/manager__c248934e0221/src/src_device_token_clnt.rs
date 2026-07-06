//! Module: src_device_token_clnt
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

fn DeviceTokenClntConstruct(tokenClnt: *mut crate::types::DeviceTokenClnt, tokenIf: *mut crate::types::IHdfDeviceToken) {
    unsafe {
        (*tokenClnt).tokenIf = tokenIf;
        (*tokenClnt).deviceInfo = core::ptr::null();
    }
}

pub extern "C" fn DeviceTokenClntNewInstance(tokenIf: *mut crate::types::IHdfDeviceToken) -> *mut crate::types::DeviceTokenClnt {
    if tokenIf.is_null() {
        let tag = b"device_token_clnt\0".as_ptr() as *const ::core::ffi::c_char;
        let msg = b"failed to create token client, tokenIf is null\0".as_ptr() as *const ::core::ffi::c_char;
        let _ = unsafe { HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510u32,
            tag,
            msg,
        ) };
        return ::core::ptr::null_mut();
    }

    let size = ::core::mem::size_of::<crate::types::DeviceTokenClnt>();
    let tokenClnt = unsafe {
        OsalMemCalloc(size.try_into().unwrap()) as *mut crate::types::DeviceTokenClnt
    };
    if !tokenClnt.is_null() {
        crate::src_device_token_clnt::DeviceTokenClntConstruct(tokenClnt, tokenIf);
    }
    tokenClnt
}

pub extern "C" fn DeviceTokenClntFreeInstance(tokenClnt: *mut crate::types::DeviceTokenClnt) {
    if !tokenClnt.is_null() {
        unsafe { OsalMemFree(tokenClnt as *mut ::core::ffi::c_void); }
    }
}

pub extern "C" fn DeviceTokenClntDelete(listEntry: *mut crate::types::HdfSListNode) {
    let token_clnt = listEntry as *mut crate::types::DeviceTokenClnt;
    if !token_clnt.is_null() {
        unsafe {
            crate::src_device_token_clnt::DeviceTokenClntFreeInstance(token_clnt);
        }
    }
}
