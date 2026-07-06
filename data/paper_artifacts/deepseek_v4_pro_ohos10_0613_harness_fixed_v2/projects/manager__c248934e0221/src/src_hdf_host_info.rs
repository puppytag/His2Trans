//! Module: src_hdf_host_info
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

pub extern "C" fn HdfHostInfoNewInstance() -> *mut crate::types::HdfHostInfo {
    unsafe { OsalMemCalloc(std::mem::size_of::<crate::types::HdfHostInfo>().try_into().unwrap()) as *mut crate::types::HdfHostInfo }
}

pub extern "C" fn HdfHostInfoFreeInstance(hostInfo: *mut crate::types::HdfHostInfo) {
    if !hostInfo.is_null() {
        unsafe {
            OsalMemFree(hostInfo as *mut ::core::ffi::c_void);
        }
    }
}

pub extern "C" fn HdfHostInfoDelete(listEntry: *mut crate::types::HdfSListNode) {
    let hostInfo = listEntry as *mut crate::types::HdfHostInfo;
    if !hostInfo.is_null() {
        unsafe {
            crate::src_hdf_host_info::HdfHostInfoFreeInstance(hostInfo);
        }
    }
}
