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
    let size = std::mem::size_of::<crate::types::DevSvcRecord>() as crate::types::size_t;
    let raw = unsafe { crate::compat::OsalMemCalloc(size) };
    raw as *mut crate::types::DevSvcRecord
}

pub extern "C" fn DevSvcRecordFreeInstance(inst: *mut crate::types::DevSvcRecord) {
    if !inst.is_null() {
        let serv_name_ptr = unsafe { (*inst).servName };
        let serv_name = serv_name_ptr as *mut ::core::ffi::c_void;
        unsafe { OsalMemFree(serv_name); }
        let serv_info_ptr = unsafe { (*inst).servInfo };
        let serv_info = serv_info_ptr as *mut ::core::ffi::c_void;
        unsafe { OsalMemFree(serv_info); }
        let iface_desc_ptr = unsafe { (*inst).interfaceDesc };
        let iface_desc = iface_desc_ptr as *mut ::core::ffi::c_void;
        unsafe { OsalMemFree(iface_desc); }
        let inst_ptr = inst as *mut ::core::ffi::c_void;
        unsafe { OsalMemFree(inst_ptr); }
    }
}
