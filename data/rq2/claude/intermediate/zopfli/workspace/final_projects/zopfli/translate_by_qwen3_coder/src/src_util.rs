//! Module: src_util
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

pub extern "C" fn ZopfliInitOptions(options: *mut crate::types::ZopfliOptions) {
    unsafe {
        *(crate::compat::c2r_field_ptr_ZopfliOptions__verbose(options as *mut ::core::ffi::c_void) as *mut i32) = 0;
        *(crate::compat::c2r_field_ptr_ZopfliOptions__verbose_more(options as *mut ::core::ffi::c_void) as *mut i32) = 0;
        *(crate::compat::c2r_field_ptr_ZopfliOptions__numiterations(options as *mut ::core::ffi::c_void) as *mut i32) = 15;
        *(crate::compat::c2r_field_ptr_ZopfliOptions__blocksplitting(options as *mut ::core::ffi::c_void) as *mut i32) = 1;
        *(crate::compat::c2r_field_ptr_ZopfliOptions__blocksplittinglast(options as *mut ::core::ffi::c_void) as *mut i32) = 0;
        *(crate::compat::c2r_field_ptr_ZopfliOptions__blocksplittingmax(options as *mut ::core::ffi::c_void) as *mut i32) = 15;
    }
}
