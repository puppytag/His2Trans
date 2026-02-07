//! Module: src_hdf_service_record
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

pub extern "C" fn DevSvcRecordNewInstance() -> *mut crate::types::DevSvcRecord {
    unsafe {
        libc::calloc(1, std::mem::size_of::<crate::types::DevSvcRecord>()) as *mut crate::types::DevSvcRecord
    }
}

pub extern "C" fn DevSvcRecordFreeInstance(inst: *mut crate::types::DevSvcRecord) {
    if !inst.is_null() {
        unsafe {
            OsalMemFree((*inst).servName as *mut ::core::ffi::c_void);
            OsalMemFree((*inst).servInfo as *mut ::core::ffi::c_void);
            OsalMemFree((*inst).interfaceDesc as *mut ::core::ffi::c_void);
            OsalMemFree(inst as *mut ::core::ffi::c_void);
        }
    }
}
