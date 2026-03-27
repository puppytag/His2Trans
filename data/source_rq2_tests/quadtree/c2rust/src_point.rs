extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: core::ffi::c_double,
    pub y: core::ffi::c_double,
}
pub type quadtree_point_t = quadtree_point;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_new(
    mut x: core::ffi::c_double,
    mut y: core::ffi::c_double,
) -> *mut quadtree_point_t {
    let mut point: *mut quadtree_point_t = 0 as *mut quadtree_point_t;
    point = malloc(::core::mem::size_of::<quadtree_point_t>() as size_t)
        as *mut quadtree_point_t;
    if point.is_null() {
        return 0 as *mut quadtree_point_t;
    }
    (*point).x = x;
    (*point).y = y;
    return point;
}
#[no_mangle]
pub unsafe extern "C" fn quadtree_point_free(mut point: *mut quadtree_point_t) {
    free(point as *mut core::ffi::c_void);
}
