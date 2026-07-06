use crate::types::{quadtree_bounds_t, quadtree_point_t};

pub(crate) fn quadtree_bounds_extend_impl(bounds: &mut quadtree_bounds_t, x: f64, y: f64) {
    let nw_x = f64::min(x, bounds.nw_as_ref().x);
    let nw_y = f64::max(y, bounds.nw_as_ref().y);
    let se_x = f64::max(x, bounds.se_as_ref().x);
    let se_y = f64::min(y, bounds.se_as_ref().y);
    bounds.nw_as_mut().x = nw_x;
    bounds.nw_as_mut().y = nw_y;
    bounds.se_as_mut().x = se_x;
    bounds.se_as_mut().y = se_y;
    bounds.width = f64::abs(nw_x - se_x);
    bounds.height = f64::abs(nw_y - se_y);
}

/// Create a new bounds with extreme corner points.
/// Returns NULL on allocation failure (in C; Rust panics on OOM).
#[no_mangle]
pub extern "C" fn quadtree_bounds_new() -> *mut quadtree_bounds_t {
    Box::into_raw(Box::new(quadtree_bounds_t {
        nw: Box::new(quadtree_point_t { x: f64::INFINITY, y: f64::NEG_INFINITY }),
        se: Box::new(quadtree_point_t { x: f64::NEG_INFINITY, y: f64::INFINITY }),
        width: 0.0,
        height: 0.0,
    }))
}

/// Extend bounds to enclose (x, y). Does nothing if bounds is NULL.
#[no_mangle]
pub extern "C" fn quadtree_bounds_extend(bounds: *mut quadtree_bounds_t, x: f64, y: f64) {
    let b = match unsafe { bounds.as_mut() } {
        Some(b) => b,
        None => return,
    };
    quadtree_bounds_extend_impl(b, x, y);
}

/// Free bounds. Safe to call on NULL.
#[no_mangle]
pub extern "C" fn quadtree_bounds_free(bounds: *mut quadtree_bounds_t) {
    if bounds.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(bounds)); }
}
