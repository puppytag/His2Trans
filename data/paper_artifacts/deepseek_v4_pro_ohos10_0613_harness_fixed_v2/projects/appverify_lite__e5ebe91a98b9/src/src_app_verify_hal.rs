//! Module: src_app_verify_hal
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

// === C2R_FILE_STATICS_BEGIN ===
// File-scope `static` variables (internal linkage) from the original C TU.
// These are module-local by design (Scheme B).
// g_productDiffFunc is now an extern "C" variable declared in compat.rs;
// its definition is provided by native/globals.c.
// static mut g_productDiffFunc removed to avoid duplicate definition.
// === C2R_FILE_STATICS_END ===

pub extern "C" fn RegistHalFunc() {
    unsafe {
        crate::products_default_app_verify_default::RegistBaseDefaultFunc(
            ::core::ptr::null_mut(),
        );
        crate::products_ipcamera_app_verify_base::RegistProductFunc(
            ::core::ptr::null_mut(),
        );
    }
}

pub extern "C" fn InquiryDeviceUdid(udid: *mut ::core::ffi::c_uchar, size: i32) -> i32 {
    unsafe {
        if let Some(func) = g_productDiffFunc.devUdidFunc {
            func(udid, size)
        } else {
            -1
        }
    }
}
