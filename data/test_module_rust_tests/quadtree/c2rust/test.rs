// Unified RQ2 test suite for quadtree in C2Rust mode.

use crate::src_quadtree::*;

unsafe extern "C" fn noop_free(_: *mut core::ffi::c_void) {}

#[test]
fn test_quadtree_new_and_free() {
    unsafe {
        let qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
        assert!(!qt.is_null());
        assert!(!(*qt).root.is_null());
        assert_eq!((*qt).length, 0);
        (*qt).key_free = Some(noop_free);
        quadtree_free(qt);
    }
}

#[test]
fn test_quadtree_insert_and_search() {
    unsafe {
        let qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
        assert!(!qt.is_null());
        (*qt).key_free = Some(noop_free);

        let key = 0x1234usize as *mut core::ffi::c_void;
        let rc = quadtree_insert(qt, 10.0, 20.0, key);
        assert_eq!(rc, 1);
        assert_eq!((*qt).length, 1);

        let p = quadtree_search(qt, 10.0, 20.0);
        assert!(!p.is_null());
        assert_eq!((*p).x, 10.0);
        assert_eq!((*p).y, 20.0);
        quadtree_free(qt);
    }
}

#[test]
fn test_quadtree_duplicate_insert() {
    unsafe {
        let qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
        assert!(!qt.is_null());
        (*qt).key_free = Some(noop_free);

        let key1 = 0x1111usize as *mut core::ffi::c_void;
        let key2 = 0x2222usize as *mut core::ffi::c_void;
        assert_eq!(quadtree_insert(qt, 1.0, 2.0, key1), 1);
        assert_eq!(quadtree_insert(qt, 1.0, 2.0, key2), 0);
        assert_eq!((*qt).length, 1);
        let p = quadtree_search(qt, 1.0, 2.0);
        assert!(!p.is_null());
        quadtree_free(qt);
    }
}

#[test]
fn test_quadtree_multiple_points() {
    unsafe {
        let qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
        assert!(!qt.is_null());
        (*qt).key_free = Some(noop_free);

        assert_eq!(quadtree_insert(qt, 10.0, 10.0, 0xaaaausize as *mut _), 1);
        assert_eq!(quadtree_insert(qt, 90.0, 10.0, 0xbbbbusize as *mut _), 1);
        assert_eq!(quadtree_insert(qt, 50.0, 90.0, 0xccccusize as *mut _), 1);
        assert_eq!((*qt).length, 3);

        let pa = quadtree_search(qt, 10.0, 10.0);
        let pb = quadtree_search(qt, 90.0, 10.0);
        let pc = quadtree_search(qt, 50.0, 90.0);
        assert!(!pa.is_null() && !pb.is_null() && !pc.is_null());
        quadtree_free(qt);
    }
}
