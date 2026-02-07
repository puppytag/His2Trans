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
        let outer_bounds_ptr = crate::compat::c2r_field_ptr_quadtree_node_t__bounds(outer as *mut ::core::ffi::c_void);
        if outer_bounds_ptr.is_null() {
            return 0;
        }
        let nw_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__nw(outer_bounds_ptr as *mut ::core::ffi::c_void);
        let se_ptr = crate::compat::c2r_field_ptr_quadtree_bounds_t__se(outer_bounds_ptr as *mut ::core::ffi::c_void);
        if nw_ptr.is_null() || se_ptr.is_null() {
            return 0;
        }
        let nw_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(nw_ptr as *mut ::core::ffi::c_void) as *const f64;
        let nw_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(nw_ptr as *mut ::core::ffi::c_void) as *const f64;
        let se_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(se_ptr as *mut ::core::ffi::c_void) as *const f64;
        let se_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(se_ptr as *mut ::core::ffi::c_void) as *const f64;
        let it_x_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__x(it as *mut ::core::ffi::c_void) as *const f64;
        let it_y_ptr = crate::compat::c2r_field_ptr_quadtree_point_t__y(it as *mut ::core::ffi::c_void) as *const f64;
        if *nw_x_ptr < *it_x_ptr && *nw_y_ptr > *it_y_ptr && *se_x_ptr > *it_x_ptr && *se_y_ptr < *it_y_ptr {
            1
        } else {
            0
        }
    }
}

fn elision_(key: *mut crate::types::c_void) {}

fn reset_node_(tree: *mut crate::types::quadtree_t, node: *mut crate::types::quadtree_node_t) {
    let key_free_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_t__key_free(tree as *mut ::core::ffi::c_void)
    };
    let key_free_val = unsafe { *(key_free_ptr as *const *mut ::core::ffi::c_void) };
    if !key_free_val.is_null() {
        let key_free_fn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)> =
            Some(unsafe { ::core::mem::transmute(key_free_val) });
        crate::src_node::quadtree_node_reset(node, key_free_fn);
    } else {
        let elision_fn: unsafe extern "C" fn(*mut ::core::ffi::c_void) = unsafe {
            ::core::mem::transmute(elision_ as *const ())
        };
        crate::src_node::quadtree_node_reset(node, Some(elision_fn));
    }
}

fn get_quadrant_(root: *mut crate::types::quadtree_node_t, point: *mut crate::types::quadtree_point_t) -> *mut crate::types::quadtree_node_t {
    let nw_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__nw(root as *mut ::core::ffi::c_void)
    };
    let nw = nw_ptr as *mut crate::types::quadtree_node_t;
    if crate::src_quadtree::node_contains_(nw, point) != 0 {
        return nw;
    }
    let ne_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__ne(root as *mut ::core::ffi::c_void)
    };
    let ne = ne_ptr as *mut crate::types::quadtree_node_t;
    if crate::src_quadtree::node_contains_(ne, point) != 0 {
        return ne;
    }
    let sw_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__sw(root as *mut ::core::ffi::c_void)
    };
    let sw = sw_ptr as *mut crate::types::quadtree_node_t;
    if crate::src_quadtree::node_contains_(sw, point) != 0 {
        return sw;
    }
    let se_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__se(root as *mut ::core::ffi::c_void)
    };
    let se = se_ptr as *mut crate::types::quadtree_node_t;
    if crate::src_quadtree::node_contains_(se, point) != 0 {
        return se;
    }
    std::ptr::null_mut()
}

fn split_node_(tree: *mut crate::types::quadtree_t, node: *mut crate::types::quadtree_node_t) -> i32 {
    let mut nw: *mut crate::types::quadtree_node_t = std::ptr::null_mut();
    let mut ne: *mut crate::types::quadtree_node_t = std::ptr::null_mut();
    let mut sw: *mut crate::types::quadtree_node_t = std::ptr::null_mut();
    let mut se: *mut crate::types::quadtree_node_t = std::ptr::null_mut();

    let bounds_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__bounds(node as *mut ::core::ffi::c_void)
    };
    let bounds = bounds_ptr as *mut crate::types::quadtree_bounds_t;

    let nw_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_bounds_t__nw(bounds as *mut ::core::ffi::c_void)
    };
    let nw_point = nw_ptr as *mut crate::types::quadtree_point_t;

    let x = unsafe {
        *(crate::compat::c2r_field_ptr_quadtree_point_t__x(nw_point as *mut ::core::ffi::c_void) as *const f64)
    };
    let y = unsafe {
        *(crate::compat::c2r_field_ptr_quadtree_point_t__y(nw_point as *mut ::core::ffi::c_void) as *const f64)
    };

    let width_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_bounds_t__width(bounds as *mut ::core::ffi::c_void)
    };
    let height_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_bounds_t__height(bounds as *mut ::core::ffi::c_void)
    };
    let hw = unsafe { *(width_ptr as *const f64) / 2.0 };
    let hh = unsafe { *(height_ptr as *const f64) / 2.0 };

    nw = crate::src_node::quadtree_node_with_bounds(x, y - hh, x + hw, y);
    if nw.is_null() {
        return 0;
    }
    ne = crate::src_node::quadtree_node_with_bounds(x + hw, y - hh, x + hw * 2.0, y);
    if ne.is_null() {
        return 0;
    }
    sw = crate::src_node::quadtree_node_with_bounds(x, y - hh * 2.0, x + hw, y - hh);
    if sw.is_null() {
        return 0;
    }
    se = crate::src_node::quadtree_node_with_bounds(x + hw, y - hh * 2.0, x + hw * 2.0, y - hh);
    if se.is_null() {
        return 0;
    }

    unsafe {
        *(crate::compat::c2r_field_ptr_quadtree_node_t__nw(node as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_node_t) = nw;
        *(crate::compat::c2r_field_ptr_quadtree_node_t__ne(node as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_node_t) = ne;
        *(crate::compat::c2r_field_ptr_quadtree_node_t__sw(node as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_node_t) = sw;
        *(crate::compat::c2r_field_ptr_quadtree_node_t__se(node as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_node_t) = se;
    }

    let old_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__point(node as *mut ::core::ffi::c_void)
    };
    let old = old_ptr as *mut crate::types::quadtree_point_t;
    let key_ptr = unsafe {
        crate::compat::c2r_field_ptr_quadtree_node_t__key(node as *mut ::core::ffi::c_void)
    };
    let key = key_ptr as *mut ::core::ffi::c_void;

    unsafe {
        *(crate::compat::c2r_field_ptr_quadtree_node_t__point(node as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_point_t) = std::ptr::null_mut();
        *(crate::compat::c2r_field_ptr_quadtree_node_t__key(node as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_void) = std::ptr::null_mut();
    }

    crate::src_quadtree::insert_(tree, node, old, key)
}

fn find_(node: *mut crate::types::quadtree_node_t, x: f64, y: f64) -> *mut crate::types::quadtree_point_t {
    if crate::src_node::quadtree_node_isleaf(node) != 0 {
        let point_ptr = unsafe {
            crate::compat::c2r_field_ptr_quadtree_node_t__point(node as *mut ::core::ffi::c_void)
        };
        let point = point_ptr as *mut crate::types::quadtree_point_t;
        if !point.is_null() {
            let point_x = unsafe {
                crate::compat::c2r_field_ptr_quadtree_point_t__x(point as *mut ::core::ffi::c_void)
            };
            let point_y = unsafe {
                crate::compat::c2r_field_ptr_quadtree_point_t__y(point as *mut ::core::ffi::c_void)
            };
            let x_val = unsafe { *(point_x as *const f64) };
            let y_val = unsafe { *(point_y as *const f64) };
            if x_val == x && y_val == y {
                return point;
            }
        }
    } else {
        let test_x_ptr = unsafe {
            libc::malloc(std::mem::size_of::<f64>()) as *mut f64
        };
        let test_y_ptr = unsafe {
            libc::malloc(std::mem::size_of::<f64>()) as *mut f64
        };
        if !test_x_ptr.is_null() && !test_y_ptr.is_null() {
            unsafe {
                *test_x_ptr = x;
                *test_y_ptr = y;
            }
            let test_ptr = unsafe {
                libc::malloc(std::mem::size_of::<crate::types::quadtree_point_t>()) as *mut crate::types::quadtree_point_t
            };
            if !test_ptr.is_null() {
                let test_x_field = unsafe {
                    crate::compat::c2r_field_ptr_quadtree_point_t__x(test_ptr as *mut ::core::ffi::c_void)
                };
                let test_y_field = unsafe {
                    crate::compat::c2r_field_ptr_quadtree_point_t__y(test_ptr as *mut ::core::ffi::c_void)
                };
                unsafe {
                    *(test_x_field as *mut f64) = x;
                    *(test_y_field as *mut f64) = y;
                }
                let quadrant = crate::src_quadtree::get_quadrant_(node, test_ptr);
                unsafe {
                    libc::free(test_ptr as *mut libc::c_void);
                }
                let result = crate::src_quadtree::find_(quadrant, x, y);
                unsafe {
                    libc::free(test_x_ptr as *mut libc::c_void);
                    libc::free(test_y_ptr as *mut libc::c_void);
                }
                return result;
            }
            unsafe {
                libc::free(test_x_ptr as *mut libc::c_void);
                libc::free(test_y_ptr as *mut libc::c_void);
            }
        }
    }
    std::ptr::null_mut()
}

fn insert_(tree: *mut crate::types::quadtree_t, root: *mut crate::types::quadtree_node_t, point: *mut crate::types::quadtree_point_t, key: *mut std::ffi::c_void) -> i32 {
    if crate::src_node::quadtree_node_isempty(root) != 0 {
        unsafe {
            *(crate::compat::c2r_field_ptr_quadtree_node_t__point(root as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_point_t) = point;
            *(crate::compat::c2r_field_ptr_quadtree_node_t__key(root as *mut ::core::ffi::c_void) as *mut *mut std::ffi::c_void) = key;
        }
        return 1;
    } else if crate::src_node::quadtree_node_isleaf(root) != 0 {
        unsafe {
            let root_point_ptr = *(crate::compat::c2r_field_ptr_quadtree_node_t__point(root as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_point_t);
            let root_x = *(crate::compat::c2r_field_ptr_quadtree_point_t__x(root_point_ptr as *mut ::core::ffi::c_void) as *mut f64);
            let root_y = *(crate::compat::c2r_field_ptr_quadtree_point_t__y(root_point_ptr as *mut ::core::ffi::c_void) as *mut f64);
            let point_x = *(crate::compat::c2r_field_ptr_quadtree_point_t__x(point as *mut ::core::ffi::c_void) as *mut f64);
            let point_y = *(crate::compat::c2r_field_ptr_quadtree_point_t__y(point as *mut ::core::ffi::c_void) as *mut f64);
            if root_x == point_x && root_y == point_y {
                crate::src_quadtree::reset_node_(tree, root);
                *(crate::compat::c2r_field_ptr_quadtree_node_t__point(root as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_point_t) = point;
                *(crate::compat::c2r_field_ptr_quadtree_node_t__key(root as *mut ::core::ffi::c_void) as *mut *mut std::ffi::c_void) = key;
                return 0;
            } else {
                if crate::src_quadtree::split_node_(tree, root) == 0 {
                    return 0;
                }
                return crate::src_quadtree::insert_(tree, root, point, key);
            }
        }
    } else if crate::src_node::quadtree_node_ispointer(root) != 0 {
        let quadrant = crate::src_quadtree::get_quadrant_(root, point);
        if quadrant.is_null() {
            return 0;
        } else {
            return crate::src_quadtree::insert_(tree, quadrant, point, key);
        }
    }
    return 0;
}

pub extern "C" fn quadtree_new(minx: f64, miny: f64, maxx: f64, maxy: f64) -> *mut crate::types::quadtree_t {
    let tree = unsafe { libc::malloc(std::mem::size_of::<crate::types::quadtree_t>()) } as *mut crate::types::quadtree_t;
    if tree.is_null() {
        return std::ptr::null_mut();
    }
    let root = crate::src_node::quadtree_node_with_bounds(minx, miny, maxx, maxy);
    if root.is_null() {
        unsafe { libc::free(tree as *mut libc::c_void); }
        return std::ptr::null_mut();
    }
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
    let root = root_ptr as *mut crate::types::quadtree_node_t;
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
        *(length_ptr as *mut u64) += 1;
    }
    1
}

pub extern "C" fn quadtree_search(tree: *mut crate::types::quadtree_t, x: f64, y: f64) -> *mut crate::types::quadtree_point_t {
    unsafe {
        let root_ptr = crate::compat::c2r_field_ptr_quadtree_t__root(tree as *mut ::core::ffi::c_void);
        let root = root_ptr as *mut crate::types::quadtree_node_t;
        crate::src_quadtree::find_(root, x, y)
    }
}

pub extern "C" fn quadtree_free(tree: *mut crate::types::quadtree_t) {
    if tree.is_null() {
        return;
    }
    unsafe {
        let key_free_ptr = crate::compat::c2r_field_ptr_quadtree_t__key_free(tree as *mut ::core::ffi::c_void);
        let key_free = *(key_free_ptr as *mut Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>);
        let root_ptr = crate::compat::c2r_field_ptr_quadtree_t__root(tree as *mut ::core::ffi::c_void);
        let root = *(root_ptr as *mut *mut crate::types::quadtree_node_t);
        if let Some(f) = key_free {
            crate::src_node::quadtree_node_free(root, Some(f));
        } else {
            crate::src_node::quadtree_node_free(root, None);
        }
        libc::free(tree as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn quadtree_walk(root: *mut crate::types::quadtree_node_t, descent: ::core::option::Option<unsafe extern "C" fn(node: *mut crate::types::quadtree_node_t)>, ascent: ::core::option::Option<unsafe extern "C" fn(node: *mut crate::types::quadtree_node_t)>) {
    if root.is_null() {
        return;
    }
    if let Some(f) = descent {
        unsafe { f(root) };
    }
    let nw_ptr = unsafe {
        *(crate::compat::c2r_field_ptr_quadtree_node_t__nw(root as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_node_t)
    };
    if !nw_ptr.is_null() {
        crate::src_quadtree::quadtree_walk(nw_ptr, descent, ascent);
    }
    let ne_ptr = unsafe {
        *(crate::compat::c2r_field_ptr_quadtree_node_t__ne(root as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_node_t)
    };
    if !ne_ptr.is_null() {
        crate::src_quadtree::quadtree_walk(ne_ptr, descent, ascent);
    }
    let sw_ptr = unsafe {
        *(crate::compat::c2r_field_ptr_quadtree_node_t__sw(root as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_node_t)
    };
    if !sw_ptr.is_null() {
        crate::src_quadtree::quadtree_walk(sw_ptr, descent, ascent);
    }
    let se_ptr = unsafe {
        *(crate::compat::c2r_field_ptr_quadtree_node_t__se(root as *mut ::core::ffi::c_void) as *mut *mut crate::types::quadtree_node_t)
    };
    if !se_ptr.is_null() {
        crate::src_quadtree::quadtree_walk(se_ptr, descent, ascent);
    }
    if let Some(f) = ascent {
        unsafe { f(root) };
    }
}
