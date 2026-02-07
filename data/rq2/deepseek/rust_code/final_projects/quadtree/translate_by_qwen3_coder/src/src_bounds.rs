//! Module: src_bounds
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

pub extern "C" fn quadtree_bounds_extend(bounds: *mut crate::types::quadtree_bounds_t, x: f64, y: f64) {
    if bounds.is_null() {
        return;
    }
    let bounds_ptr = bounds as *mut ::core::ffi::c_void;
    unsafe {
        let nw_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__nw(bounds_ptr) as *mut *mut crate::types::quadtree_point_t;
        let nw = *nw_ptr;
        if !nw.is_null() {
            let nw_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(nw as *mut ::core::ffi::c_void) as *mut f64;
            *nw_x_ptr = crate::compat::fmin(x, *nw_x_ptr);
            let nw_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(nw as *mut ::core::ffi::c_void) as *mut f64;
            *nw_y_ptr = crate::compat::fmax(y, *nw_y_ptr);
        }
        let se_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__se(bounds_ptr) as *mut *mut crate::types::quadtree_point_t;
        let se = *se_ptr;
        if !se.is_null() {
            let se_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(se as *mut ::core::ffi::c_void) as *mut f64;
            *se_x_ptr = crate::compat::fmax(x, *se_x_ptr);
            let se_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(se as *mut ::core::ffi::c_void) as *mut f64;
            *se_y_ptr = crate::compat::fmin(y, *se_y_ptr);
        }
        if !nw.is_null() && !se.is_null() {
            let nw_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(nw as *mut ::core::ffi::c_void) as *mut f64;
            let nw_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(nw as *mut ::core::ffi::c_void) as *mut f64;
            let se_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(se as *mut ::core::ffi::c_void) as *mut f64;
            let se_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(se as *mut ::core::ffi::c_void) as *mut f64;
            let width_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__width(bounds_ptr) as *mut f64;
            *width_ptr = crate::compat::fabs(*nw_x_ptr - *se_x_ptr);
            let height_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__height(bounds_ptr) as *mut f64;
            *height_ptr = crate::compat::fabs(*nw_y_ptr - *se_y_ptr);
        }
    }
}

pub extern "C" fn quadtree_bounds_free(bounds: *mut crate::types::quadtree_bounds_t) {
    if bounds.is_null() {
        return;
    }
    unsafe {
        let nw_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__nw(bounds as *mut ::core::ffi::c_void) as *mut crate::types::quadtree_point_t;
        if !nw_ptr.is_null() {
            crate::src_point::quadtree_point_free(nw_ptr);
        }
        let se_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__se(bounds as *mut ::core::ffi::c_void) as *mut crate::types::quadtree_point_t;
        if !se_ptr.is_null() {
            crate::src_point::quadtree_point_free(se_ptr);
        }
        libc::free(bounds as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn quadtree_bounds_new() -> *mut crate::types::quadtree_bounds_t {
    let bounds = unsafe { libc::malloc(std::mem::size_of::<crate::types::quadtree_bounds_t>()) as *mut crate::types::quadtree_bounds_t };
    if bounds.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*bounds).nw = crate::src_point::quadtree_point_new(f64::INFINITY, -f64::INFINITY) as *mut ::core::ffi::c_void;
        (*bounds).se = crate::src_point::quadtree_point_new(-f64::INFINITY, f64::INFINITY) as *mut ::core::ffi::c_void;
        (*bounds).width = 0;
        (*bounds).height = 0;
    }
    bounds
}
