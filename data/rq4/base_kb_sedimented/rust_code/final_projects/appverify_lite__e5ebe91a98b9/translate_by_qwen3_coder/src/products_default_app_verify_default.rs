//! Module: products_default_app_verify_default
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

fn GetUdid(udid: *mut u8, size: i32) -> i32 {
    let ret = unsafe { crate::compat::GetDevUdid(udid as *mut ::core::ffi::c_char, size) };
    ret
}

pub extern "C" fn RegistBaseDefaultFunc(productFunc: *mut crate::types::ProductDiff) {
    if productFunc.is_null() {
        return;
    }
    unsafe extern "C" fn get_udid_wrapper(udid: *mut ::core::ffi::c_uchar, size: i32) -> i32 {
        GetUdid(udid, size)
    }
    unsafe {
        (*productFunc).devUdidFunc = Some(get_udid_wrapper);
    }
}
