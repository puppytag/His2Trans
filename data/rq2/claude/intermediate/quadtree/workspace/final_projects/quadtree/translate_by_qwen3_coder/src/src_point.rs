//! Module: src_point
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

pub extern "C" fn quadtree_point_new(x: f64, y: f64) -> *mut crate::types::quadtree_point_t {
    let point = unsafe { libc::malloc(std::mem::size_of::<crate::types::quadtree_point_t>()) } as *mut crate::types::quadtree_point_t;
    if point.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(point as *mut ::core::ffi::c_void) as *mut f64;
        *x_ptr = x;
        let y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(point as *mut ::core::ffi::c_void) as *mut f64;
        *y_ptr = y;
    }
    point
}

pub extern "C" fn quadtree_point_free(point: *mut crate::types::quadtree_point_t) {
    unsafe {
        libc::free(point as *mut ::core::ffi::c_void);
    }
}
