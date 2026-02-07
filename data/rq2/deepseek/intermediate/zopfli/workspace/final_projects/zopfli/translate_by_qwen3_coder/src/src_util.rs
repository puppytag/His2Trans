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
    if options.is_null() {
        return;
    }
    unsafe {
        let options_ptr = options as *mut ::core::ffi::c_void;
        let verbose_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__numiterations(options_ptr) as *mut i32;
        *verbose_ptr = 0;
        let verbose_more_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__numiterations(options_ptr) as *mut i32;
        *verbose_more_ptr = 0;
        let numiterations_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__numiterations(options_ptr) as *mut i32;
        *numiterations_ptr = 15;
        let blocksplitting_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__blocksplittingmax(options_ptr) as *mut i32;
        *blocksplitting_ptr = 1;
        let blocksplittinglast_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__blocksplittingmax(options_ptr) as *mut i32;
        *blocksplittinglast_ptr = 0;
        let blocksplittingmax_ptr = crate::compat::c2r_field_ptr_ZopfliOptions__blocksplittingmax(options_ptr) as *mut i32;
        *blocksplittingmax_ptr = 15;
    }
}
