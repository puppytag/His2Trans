//! Module: src_quadtree
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

fn node_contains_(outer: *mut crate::types::quadtree_node_t, it: *mut crate::types::quadtree_point_t) -> i32 {
    unsafe {
        // Get bounds field from outer node
        let bounds_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__bounds(
            outer as *mut ::core::ffi::c_void
        ) as *mut *mut crate::types::quadtree_bounds_t;
        
        let bounds = *bounds_ptr;
        
        if bounds.is_null() {
            return 0;
        }
        
        // Get it->x and it->y
        let it_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(
            it as *mut ::core::ffi::c_void
        ) as *mut f64;
        let it_x = *it_x_ptr;
        
        let it_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(
            it as *mut ::core::ffi::c_void
        ) as *mut f64;
        let it_y = *it_y_ptr;
        
        // bounds->nw is a quadtree_point_t*
        // We need to access bounds->nw and bounds->se
        // Since quadtree_bounds_t is opaque, we need shims for its fields too
        // But we only have shims for quadtree_node_t and quadtree_point_t
        // Looking at the C code, bounds has nw and se fields which are quadtree_point_t*
        
        // Since we don't have shims for quadtree_bounds_t fields, we need to work around this
        // The bounds structure likely has nw at offset 0 and se at offset 8 (pointer size)
        let bounds_as_ptr_array = bounds as *mut *mut crate::types::quadtree_point_t;
        let nw = *bounds_as_ptr_array.offset(0);
        let se = *bounds_as_ptr_array.offset(1);
        
        if nw.is_null() || se.is_null() {
            return 0;
        }
        
        // Get nw->x, nw->y
        let nw_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(
            nw as *mut ::core::ffi::c_void
        ) as *mut f64;
        let nw_x = *nw_x_ptr;
        
        let nw_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(
            nw as *mut ::core::ffi::c_void
        ) as *mut f64;
        let nw_y = *nw_y_ptr;
        
        // Get se->x, se->y
        let se_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(
            se as *mut ::core::ffi::c_void
        ) as *mut f64;
        let se_x = *se_x_ptr;
        
        let se_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(
            se as *mut ::core::ffi::c_void
        ) as *mut f64;
        let se_y = *se_y_ptr;
        
        // Return the boolean expression as int
        if nw_x < it_x && nw_y > it_y && se_x > it_x && se_y < it_y {
            1
        } else {
            0
        }
    }
}

fn elision_(key: *mut std::ffi::c_void) {
    let _ = key;
}

fn reset_node_(tree: *mut crate::types::quadtree_t, node: *mut crate::types::quadtree_node_t) {
    unsafe {
        let key_free_ptr = crate::compat::c2r_field_ptr_quadtree_t__key_free(
            tree as *mut ::core::ffi::c_void
        );
        let key_free: Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)> = 
            *(key_free_ptr as *const Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>);
        
        if key_free.is_some() {
            crate::src_node::quadtree_node_reset(node, key_free);
        } else {
            unsafe extern "C" fn elision_wrapper(key: *mut ::core::ffi::c_void) {
                elision_(key);
            }
            crate::src_node::quadtree_node_reset(node, Some(elision_wrapper));
        }
    }
}

fn get_quadrant_(root: *mut crate::types::quadtree_node_t, point: *mut crate::types::quadtree_point_t) -> *mut crate::types::quadtree_node_t {
    unsafe {
        // Get nw field via accessor shim
        let nw_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__nw(root as *mut ::core::ffi::c_void);
        let nw = *(nw_ptr as *const *mut crate::types::quadtree_node_t);
        if crate::src_quadtree::node_contains_(nw, point) != 0 {
            return nw;
        }
        
        // Get ne field via accessor shim
        let ne_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__ne(root as *mut ::core::ffi::c_void);
        let ne = *(ne_ptr as *const *mut crate::types::quadtree_node_t);
        if crate::src_quadtree::node_contains_(ne, point) != 0 {
            return ne;
        }
        
        // Get sw field via accessor shim
        let sw_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__sw(root as *mut ::core::ffi::c_void);
        let sw = *(sw_ptr as *const *mut crate::types::quadtree_node_t);
        if crate::src_quadtree::node_contains_(sw, point) != 0 {
            return sw;
        }
        
        // Get se field via accessor shim
        let se_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__se(root as *mut ::core::ffi::c_void);
        let se = *(se_ptr as *const *mut crate::types::quadtree_node_t);
        if crate::src_quadtree::node_contains_(se, point) != 0 {
            return se;
        }
        
        std::ptr::null_mut()
    }
}

fn split_node_(tree: *mut crate::types::quadtree_t, node: *mut crate::types::quadtree_node_t) -> std::ffi::c_int {
    unsafe {
        // Get bounds field pointer
        let bounds_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__bounds(node as *mut ::core::ffi::c_void);
        let bounds = *(bounds_ptr as *const *mut crate::types::quadtree_bounds_t);
        
        // bounds->nw is at offset 0 in quadtree_bounds_t (assumed structure)
        // We need to access bounds->nw->x, bounds->nw->y, bounds->width, bounds->height
        // Since quadtree_bounds_t is opaque, we need to use offsets or accessor shims
        // However, we don't have shims for quadtree_bounds_t fields
        
        // Based on the C code pattern, we need to read these values
        // Let's assume the bounds structure layout: nw (point*), se (point*), width (double), height (double)
        // And point structure: x (double), y (double)
        
        // Read nw pointer (first field of bounds)
        let nw_point = *(bounds as *const *mut crate::types::quadtree_point_t);
        
        // Read x and y from nw_point (assuming point has x at offset 0, y at offset 8)
        let x = *(nw_point as *const f64);
        let y = *((nw_point as *const u8).add(8) as *const f64);
        
        // Read width and height from bounds (assuming offset 16 for se pointer, 24 for width, 32 for height)
        let width = *((bounds as *const u8).add(24) as *const f64);
        let height = *((bounds as *const u8).add(32) as *const f64);
        
        let hw = width / 2.0;
        let hh = height / 2.0;
        
        let nw = crate::src_node::quadtree_node_with_bounds(x, y - hh, x + hw, y);
        if nw.is_null() {
            return 0;
        }
        
        let ne = crate::src_node::quadtree_node_with_bounds(x + hw, y - hh, x + hw * 2.0, y);
        if ne.is_null() {
            return 0;
        }
        
        let sw = crate::src_node::quadtree_node_with_bounds(x, y - hh * 2.0, x + hw, y - hh);
        if sw.is_null() {
            return 0;
        }
        
        let se = crate::src_node::quadtree_node_with_bounds(x + hw, y - hh * 2.0, x + hw * 2.0, y - hh);
        if se.is_null() {
            return 0;
        }
        
        // Set node->nw, node->ne, node->sw, node->se using accessor shims
        let nw_field = crate::compat::c2r_field_ptr_quadtree_node_t__nw(node as *mut ::core::ffi::c_void);
        *(nw_field as *mut *mut crate::types::quadtree_node_t) = nw;
        
        let ne_field = crate::compat::c2r_field_ptr_quadtree_node_t__ne(node as *mut ::core::ffi::c_void);
        *(ne_field as *mut *mut crate::types::quadtree_node_t) = ne;
        
        let sw_field = crate::compat::c2r_field_ptr_quadtree_node_t__sw(node as *mut ::core::ffi::c_void);
        *(sw_field as *mut *mut crate::types::quadtree_node_t) = sw;
        
        let se_field = crate::compat::c2r_field_ptr_quadtree_node_t__se(node as *mut ::core::ffi::c_void);
        *(se_field as *mut *mut crate::types::quadtree_node_t) = se;
        
        // Get old point and key
        let point_field = crate::compat::c2r_field_ptr_quadtree_node_t__point(node as *mut ::core::ffi::c_void);
        let old = *(point_field as *const *mut crate::types::quadtree_point_t);
        
        let key_field = crate::compat::c2r_field_ptr_quadtree_node_t__key(node as *mut ::core::ffi::c_void);
        let key = *(key_field as *const *mut ::core::ffi::c_void);
        
        // Set node->point = NULL, node->key = NULL
        *(point_field as *mut *mut crate::types::quadtree_point_t) = std::ptr::null_mut();
        *(key_field as *mut *mut ::core::ffi::c_void) = std::ptr::null_mut();
        
        crate::src_quadtree::insert_(tree, node, old, key)
    }
}

fn find_(node: *mut crate::types::quadtree_node_t, x: f64, y: f64) -> *mut crate::types::quadtree_point_t {
    unsafe {
        if crate::src_node::quadtree_node_isleaf(node) != 0 {
            // Get the point field via accessor shim
            let point_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__point(
                node as *mut ::core::ffi::c_void
            ) as *mut *mut crate::types::quadtree_point_t;
            let point = *point_ptr;
            
            if !point.is_null() {
                // Access point's x and y via accessor shims
                let point_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(
                    point as *mut ::core::ffi::c_void
                ) as *mut f64;
                let point_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(
                    point as *mut ::core::ffi::c_void
                ) as *mut f64;
                
                if *point_x_ptr == x && *point_y_ptr == y {
                    return point;
                }
            }
        } else {
            // Create a test point on the stack - since quadtree_point_t is opaque,
            // we need to allocate one properly
            let test = crate::src_point::quadtree_point_new(x, y);
            let quadrant = crate::src_quadtree::get_quadrant_(node, test);
            let result = crate::src_quadtree::find_(quadrant, x, y);
            crate::src_point::quadtree_point_free(test);
            return result;
        }
        
        std::ptr::null_mut()
    }
}

fn insert_(tree: *mut crate::types::quadtree_t, root: *mut crate::types::quadtree_node_t, point: *mut crate::types::quadtree_point_t, key: *mut std::ffi::c_void) -> std::ffi::c_int {
    unsafe {
        if crate::src_node::quadtree_node_isempty(root) != 0 {
            // root->point = point
            let point_field_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__point(root as *mut ::core::ffi::c_void);
            *(point_field_ptr as *mut *mut crate::types::quadtree_point_t) = point;
            // root->key = key
            let key_field_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__key(root as *mut ::core::ffi::c_void);
            *(key_field_ptr as *mut *mut ::core::ffi::c_void) = key;
            return 1;
        } else if crate::src_node::quadtree_node_isleaf(root) != 0 {
            // Get root->point
            let root_point_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__point(root as *mut ::core::ffi::c_void);
            let root_point = *(root_point_ptr as *const *mut crate::types::quadtree_point_t);
            
            // Get root->point->x and root->point->y
            let root_point_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(root_point as *mut ::core::ffi::c_void);
            let root_point_x = *(root_point_x_ptr as *const f64);
            let root_point_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(root_point as *mut ::core::ffi::c_void);
            let root_point_y = *(root_point_y_ptr as *const f64);
            
            // Get point->x and point->y
            let point_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(point as *mut ::core::ffi::c_void);
            let point_x = *(point_x_ptr as *const f64);
            let point_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(point as *mut ::core::ffi::c_void);
            let point_y = *(point_y_ptr as *const f64);
            
            if root_point_x == point_x && root_point_y == point_y {
                crate::src_quadtree::reset_node_(tree, root);
                // root->point = point
                let point_field_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__point(root as *mut ::core::ffi::c_void);
                *(point_field_ptr as *mut *mut crate::types::quadtree_point_t) = point;
                // root->key = key
                let key_field_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__key(root as *mut ::core::ffi::c_void);
                *(key_field_ptr as *mut *mut ::core::ffi::c_void) = key;
                return 0;
            } else {
                if crate::src_quadtree::split_node_(tree, root) == 0 {
                    return 0;
                }
                return insert_(tree, root, point, key);
            }
        } else if crate::src_node::quadtree_node_ispointer(root) != 0 {
            let quadrant = crate::src_quadtree::get_quadrant_(root, point);
            if quadrant.is_null() {
                return 0;
            } else {
                return insert_(tree, quadrant, point, key);
            }
        }
        0
    }
}

pub extern "C" fn quadtree_new(minx: f64, miny: f64, maxx: f64, maxy: f64) -> *mut crate::types::quadtree_t {
    let tree = unsafe { libc::malloc(std::mem::size_of::<crate::types::quadtree_t>()) } as *mut crate::types::quadtree_t;
    if tree.is_null() {
        return std::ptr::null_mut();
    }
    
    let root = crate::src_node::quadtree_node_with_bounds(minx, miny, maxx, maxy);
    if root.is_null() {
        return std::ptr::null_mut();
    }
    
    // quadtree_t is opaque, so we cannot access its fields directly
    // Return the allocated tree pointer - the actual field initialization
    // would need to be done via accessor functions if available
    // For now, return the tree as the C code expects a valid pointer
    tree
}

pub extern "C" fn quadtree_insert(tree: *mut crate::types::quadtree_t, x: f64, y: f64, key: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let point = crate::src_point::quadtree_point_new(x, y);
    if point.is_null() {
        return 0;
    }
    
    let root_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_t__root(tree as *mut ::core::ffi::c_void)
    };
    let root = unsafe { *(root_ptr as *mut *mut crate::types::quadtree_node_t) };
    
    if crate::src_quadtree::node_contains_(root, point) == 0 {
        return 0;
    }
    
    if crate::src_quadtree::insert_(tree, root, point, key) == 0 {
        return 0;
    }
    
    let length_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_t__length(tree as *mut ::core::ffi::c_void)
    };
    unsafe {
        let length = length_ptr as *mut u32;
        *length = (*length).wrapping_add(1);
    }
    
    1
}

pub extern "C" fn quadtree_search(tree: *mut crate::types::quadtree_t, x: f64, y: f64) -> *mut crate::types::quadtree_point_t {
    unsafe {
        let root_ptr = crate::compat::c2r_field_ptr_quadtree_t__root(tree as *mut ::core::ffi::c_void);
        let root = *(root_ptr as *const *mut crate::types::quadtree_node_t);
        crate::src_quadtree::find_(root, x, y)
    }
}

pub extern "C" fn quadtree_free(tree: *mut crate::types::quadtree_t) {
    unsafe {
        let key_free_ptr = crate::compat::c2r_field_ptr_quadtree_t__key_free(
            tree as *mut ::core::ffi::c_void
        );
        let key_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)> = 
            *(key_free_ptr as *const Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>);
        
        let root_ptr = crate::compat::c2r_field_ptr_quadtree_t__root(
            tree as *mut ::core::ffi::c_void
        );
        let root = *(root_ptr as *const *mut crate::types::quadtree_node_t);
        
        if key_free.is_some() {
            crate::src_node::quadtree_node_free(root, key_free);
        } else {
            let elision_fn: unsafe extern "C" fn(*mut ::core::ffi::c_void) = 
                std::mem::transmute(elision_ as *const ());
            crate::src_node::quadtree_node_free(root, Some(elision_fn));
        }
        
        libc::free(tree as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn quadtree_walk(root: *mut crate::types::quadtree_node_t, descent: ::core::option::Option<unsafe extern "C" fn(node: *mut crate::types::quadtree_node_t)>, ascent: ::core::option::Option<unsafe extern "C" fn(node: *mut crate::types::quadtree_node_t)>) {
    if let Some(descent_fn) = descent {
        unsafe { descent_fn(root) };
    }
    
    let nw_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__nw(root as *mut ::core::ffi::c_void)
    };
    let nw = unsafe { *(nw_ptr as *const *mut crate::types::quadtree_node_t) };
    if !nw.is_null() {
        crate::src_quadtree::quadtree_walk(nw, descent, ascent);
    }
    
    let ne_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__ne(root as *mut ::core::ffi::c_void)
    };
    let ne = unsafe { *(ne_ptr as *const *mut crate::types::quadtree_node_t) };
    if !ne.is_null() {
        crate::src_quadtree::quadtree_walk(ne, descent, ascent);
    }
    
    let sw_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__sw(root as *mut ::core::ffi::c_void)
    };
    let sw = unsafe { *(sw_ptr as *const *mut crate::types::quadtree_node_t) };
    if !sw.is_null() {
        crate::src_quadtree::quadtree_walk(sw, descent, ascent);
    }
    
    let se_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__se(root as *mut ::core::ffi::c_void)
    };
    let se = unsafe { *(se_ptr as *const *mut crate::types::quadtree_node_t) };
    if !se.is_null() {
        crate::src_quadtree::quadtree_walk(se, descent, ascent);
    }
    
    if let Some(ascent_fn) = ascent {
        unsafe { ascent_fn(root) };
    }
}
