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
/// C: static ProductDiff g_productDiffFunc
static mut g_productDiffFunc: crate::types::ProductDiff = unsafe { core::mem::MaybeUninit::<crate::types::ProductDiff>::zeroed().assume_init() };

// === C2R_FILE_STATICS_END ===

pub extern "C" fn RegistHalFunc() {
    unsafe {
        crate::products_default_app_verify_default::RegistBaseDefaultFunc(&mut g_productDiffFunc);
        crate::products_ipcamera_app_verify_base::RegistProductFunc(&mut g_productDiffFunc);
    }
}

pub extern "C" fn InquiryDeviceUdid(udid: *mut ::core::ffi::c_uchar, size: i32) -> i32 {
    unsafe {
        if g_productDiffFunc.devUdidFunc.is_none() {
            return INQUIRY_UDID_ERROR;
        }
        if let Some(f) = g_productDiffFunc.devUdidFunc {
            f(udid, size)
        } else {
            INQUIRY_UDID_ERROR
        }
    }
}
