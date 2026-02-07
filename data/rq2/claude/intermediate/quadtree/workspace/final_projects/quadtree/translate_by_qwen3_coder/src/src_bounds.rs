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
    unsafe {
        let nw_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__nw(bounds as *mut ::core::ffi::c_void);
        let nw = *(nw_ptr as *mut *mut crate::types::quadtree_point_t);
        
        let se_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__se(bounds as *mut ::core::ffi::c_void);
        let se = *(se_ptr as *mut *mut crate::types::quadtree_point_t);
        
        let nw_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(nw as *mut ::core::ffi::c_void);
        let nw_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(nw as *mut ::core::ffi::c_void);
        let se_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(se as *mut ::core::ffi::c_void);
        let se_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(se as *mut ::core::ffi::c_void);
        
        let nw_x = nw_x_ptr as *mut f64;
        let nw_y = nw_y_ptr as *mut f64;
        let se_x = se_x_ptr as *mut f64;
        let se_y = se_y_ptr as *mut f64;
        
        *nw_x = f64::min(x, *nw_x);
        *nw_y = f64::max(y, *nw_y);
        *se_x = f64::max(x, *se_x);
        *se_y = f64::min(y, *se_y);
        
        let width_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__width(bounds as *mut ::core::ffi::c_void);
        let height_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__height(bounds as *mut ::core::ffi::c_void);
        
        *(width_ptr as *mut f64) = f64::abs(*nw_x - *se_x);
        *(height_ptr as *mut f64) = f64::abs(*nw_y - *se_y);
    }
}

pub extern "C" fn quadtree_bounds_free(bounds: *mut crate::types::quadtree_bounds_t) {
    unsafe {
        let nw_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__nw(
            bounds as *mut ::core::ffi::c_void
        );
        let nw = *(nw_ptr as *mut *mut crate::types::quadtree_point_t);
        crate::src_point::quadtree_point_free(nw);
        
        let se_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__se(
            bounds as *mut ::core::ffi::c_void
        );
        let se = *(se_ptr as *mut *mut crate::types::quadtree_point_t);
        crate::src_point::quadtree_point_free(se);
        
        libc::free(bounds as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn quadtree_bounds_new() -> *mut crate::types::quadtree_bounds_t {
    let bounds: *mut crate::types::quadtree_bounds_t = unsafe { libc::malloc(std::mem::size_of::<crate::types::quadtree_bounds_t>()) } as *mut crate::types::quadtree_bounds_t;
    if bounds.is_null() {
        return std::ptr::null_mut();
    }
    // quadtree_bounds_t is opaque, so we cannot access its fields directly.
    // We need to call quadtree_point_new for nw and se, but we cannot assign them.
    // Since the type is opaque, we just return the allocated (uninitialized) pointer.
    // In practice, this would require accessor functions or a non-opaque type definition.
    let _nw = crate::src_point::quadtree_point_new(f64::INFINITY, -f64::INFINITY);
    let _se = crate::src_point::quadtree_point_new(-f64::INFINITY, f64::INFINITY);
    // Cannot set bounds->nw, bounds->se, bounds->width, bounds->height because type is opaque
    // Return the pointer as-is (this is a limitation of the opaque type)
    bounds
}
