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

fn GetUdid(udid: *mut u8, size: i32)-> i32 {
    unsafe { GetDevUdid(udid as *mut ::core::ffi::c_char, size) }
}

pub extern "C" fn RegistBaseDefaultFunc(productFunc: *mut crate::types::ProductDiff) {
    unsafe {
        let f: fn(*mut u8, i32) -> i32 = crate::products_default_app_verify_default::GetUdid;
        (*productFunc).devUdidFunc = Some(std::mem::transmute::<
            fn(*mut u8, i32) -> i32,
            unsafe extern "C" fn(*mut ::core::ffi::c_uchar, i32) -> i32,
        >(f));
    }
}
