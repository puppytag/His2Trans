use crate::types::{quadtree_node_t, c_void};
use crate::src_bounds::quadtree_bounds_extend_impl;
use std::ptr;

pub(crate) fn node_reset_impl(
    node: &mut quadtree_node_t,
    key_free: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    node.point = None; // drop point Box
    if let Some(free_fn) = key_free {
        if !node.key.is_null() {
            unsafe { free_fn(node.key); }
        }
    }
    node.key = ptr::null_mut();
}

fn node_free_impl(
    node: &mut quadtree_node_t,
    value_free: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    if let Some(child) = node.nw_as_mut() { node_free_impl(child, value_free); }
    if let Some(child) = node.ne_as_mut() { node_free_impl(child, value_free); }
    if let Some(child) = node.sw_as_mut() { node_free_impl(child, value_free); }
    if let Some(child) = node.se_as_mut() { node_free_impl(child, value_free); }
    node_reset_impl(node, value_free);
}

// --- Public API ---

#[no_mangle]
pub extern "C" fn quadtree_node_new() -> *mut quadtree_node_t {
    Box::into_raw(Box::new(quadtree_node_t {
        ne: None, nw: None, se: None, sw: None,
        bounds: None, point: None, key: ptr::null_mut(),
    }))
}

#[no_mangle]
pub extern "C" fn quadtree_node_free(
    node: *mut quadtree_node_t,
    value_free: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    if node.is_null() { return; }
    {
        let node_ref = unsafe { &mut *node };
        node_free_impl(node_ref, value_free);
    }
    let _ = unsafe { Box::from_raw(node) };
}

#[no_mangle]
pub extern "C" fn quadtree_node_ispointer(node: *mut quadtree_node_t) -> i32 {
    match unsafe { node.as_ref() } {
        Some(n) => n.is_pointer() as i32,
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn quadtree_node_isempty(node: *mut quadtree_node_t) -> i32 {
    match unsafe { node.as_ref() } {
        Some(n) => n.is_empty() as i32,
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn quadtree_node_isleaf(node: *mut quadtree_node_t) -> i32 {
    match unsafe { node.as_ref() } {
        Some(n) => n.is_leaf() as i32,
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn quadtree_node_reset(
    node: *mut quadtree_node_t,
    key_free: Option<unsafe extern "C" fn(*mut c_void)>,
) {
    if let Some(n) = unsafe { node.as_mut() } {
        node_reset_impl(n, key_free);
    }
}

pub(crate) fn quadtree_node_with_bounds_box(
    minx: f64, miny: f64, maxx: f64, maxy: f64,
) -> Option<Box<quadtree_node_t>> {
    use crate::types::{quadtree_point_t, quadtree_bounds_t};
    let mut node = Box::new(quadtree_node_t {
        ne: None, nw: None, se: None, sw: None,
        bounds: None, point: None, key: ptr::null_mut(),
    });
    let mut bounds = Box::new(quadtree_bounds_t {
        nw: Box::new(quadtree_point_t { x: f64::INFINITY, y: f64::NEG_INFINITY }),
        se: Box::new(quadtree_point_t { x: f64::NEG_INFINITY, y: f64::INFINITY }),
        width: 0.0, height: 0.0,
    });
    quadtree_bounds_extend_impl(&mut bounds, maxx, maxy);
    quadtree_bounds_extend_impl(&mut bounds, minx, miny);
    node.bounds = Some(bounds);
    Some(node)
}

#[no_mangle]
pub extern "C" fn quadtree_node_with_bounds(
    minx: f64, miny: f64, maxx: f64, maxy: f64,
) -> *mut quadtree_node_t {
    match quadtree_node_with_bounds_box(minx, miny, maxx, maxy) {
        Some(n) => Box::into_raw(n),
        None => ptr::null_mut(),
    }
}
