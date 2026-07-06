use crate::types::quadtree_point_t;

/// Create a new point with given coordinates.
#[no_mangle]
pub extern "C" fn quadtree_point_new(x: f64, y: f64) -> *mut quadtree_point_t {
    Box::into_raw(Box::new(quadtree_point_t { x, y }))
}

/// Free a point. Safe to call on NULL.
#[no_mangle]
pub extern "C" fn quadtree_point_free(point: *mut quadtree_point_t) {
    if !point.is_null() {
        unsafe {
            drop(Box::from_raw(point));
        }
    }
}
