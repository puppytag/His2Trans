//! Module: src_node
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

pub extern "C" fn quadtree_node_ispointer(node: *mut crate::types::quadtree_node_t) -> ::core::ffi::c_int {
    if node.is_null() {
        return 0;
    }
    let node_ptr = node as *mut ::core::ffi::c_void;
    let nw_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__nw(node_ptr) as *mut ::core::ffi::c_void
    };
    let ne_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__ne(node_ptr) as *mut ::core::ffi::c_void
    };
    let sw_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__sw(node_ptr) as *mut ::core::ffi::c_void
    };
    let se_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__se(node_ptr) as *mut ::core::ffi::c_void
    };
    if !nw_ptr.is_null() && !ne_ptr.is_null() && !sw_ptr.is_null() && !se_ptr.is_null() {
        let leaf_result = crate::src_node::quadtree_node_isleaf(node);
        if leaf_result == 0 {
            return 1;
        }
    }
    0
}

pub extern "C" fn quadtree_node_isempty(node: *mut crate::types::quadtree_node_t) -> ::core::ffi::c_int {
    if node.is_null() {
        return 0;
    }
    let nw_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__nw(node as *mut ::core::ffi::c_void)
    };
    let ne_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__ne(node as *mut ::core::ffi::c_void)
    };
    let sw_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__sw(node as *mut ::core::ffi::c_void)
    };
    let se_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__se(node as *mut ::core::ffi::c_void)
    };
    let nw = unsafe { *(nw_ptr as *const *mut crate::types::quadtree_node_t) };
    let ne = unsafe { *(ne_ptr as *const *mut crate::types::quadtree_node_t) };
    let sw = unsafe { *(sw_ptr as *const *mut crate::types::quadtree_node_t) };
    let se = unsafe { *(se_ptr as *const *mut crate::types::quadtree_node_t) };
    let is_leaf = crate::src_node::quadtree_node_isleaf(node);
    (nw.is_null() && ne.is_null() && sw.is_null() && se.is_null() && is_leaf == 0) as ::core::ffi::c_int
}

pub extern "C" fn quadtree_node_isleaf(node: *mut crate::types::quadtree_node_t) -> ::core::ffi::c_int {
    if node.is_null() {
        return 0;
    }
    let point_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__point(node as *mut ::core::ffi::c_void)
    };
    if point_ptr.is_null() {
        0
    } else {
        1
    }
}

pub extern "C" fn quadtree_node_reset(node: *mut crate::types::quadtree_node_t, key_free: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>) {
    if node.is_null() {
        return;
    }
    unsafe {
        let point_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__point(node as *mut ::core::ffi::c_void) as *mut crate::types::quadtree_point_t;
        if !point_ptr.is_null() {
            crate::src_point::quadtree_point_free(point_ptr);
        }
        if let Some(f) = key_free {
            let key_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__key(node as *mut ::core::ffi::c_void);
            if !key_ptr.is_null() {
                f(key_ptr);
            }
        }
    }
}

pub extern "C" fn quadtree_node_new() -> *mut crate::types::quadtree_node_t {
    let node = unsafe { libc::malloc(std::mem::size_of::<crate::types::quadtree_node_t>()) } as *mut crate::types::quadtree_node_t;
    if node.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        std::ptr::write_bytes(node, 0, 1);
    }
    node
}

pub extern "C" fn quadtree_node_with_bounds(minx: f64, miny: f64, maxx: f64, maxy: f64) -> *mut crate::types::quadtree_node_t {
    let node = crate::src_node::quadtree_node_new();
    if node.is_null() {
        return std::ptr::null_mut();
    }
    let bounds = crate::src_bounds::quadtree_bounds_new();
    if bounds.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let bounds_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__bounds(node as *mut ::core::ffi::c_void);
        *(bounds_ptr as *mut *mut crate::types::quadtree_bounds_t) = bounds;
    }
    crate::src_bounds::quadtree_bounds_extend(bounds, maxx, maxy);
    crate::src_bounds::quadtree_bounds_extend(bounds, minx, miny);
    node
}

pub extern "C" fn quadtree_node_free(node: *mut crate::types::quadtree_node_t, value_free: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>) {
    if node.is_null() {
        return;
    }
    unsafe {
        let nw = *((node as *mut *mut crate::types::quadtree_node_t).offset(0));
        if !nw.is_null() {
            crate::src_node::quadtree_node_free(nw, value_free);
        }
        let ne = *((node as *mut *mut crate::types::quadtree_node_t).offset(1));
        if !ne.is_null() {
            crate::src_node::quadtree_node_free(ne, value_free);
        }
        let sw = *((node as *mut *mut crate::types::quadtree_node_t).offset(2));
        if !sw.is_null() {
            crate::src_node::quadtree_node_free(sw, value_free);
        }
        let se = *((node as *mut *mut crate::types::quadtree_node_t).offset(3));
        if !se.is_null() {
            crate::src_node::quadtree_node_free(se, value_free);
        }
        let bounds = *((node as *mut *mut crate::types::quadtree_bounds_t).offset(4));
        crate::src_bounds::quadtree_bounds_free(bounds);
        crate::src_node::quadtree_node_reset(node, value_free);
        libc::free(node as *mut libc::c_void);
    }
}
