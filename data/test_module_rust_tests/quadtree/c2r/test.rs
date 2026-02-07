// C2R 框架测试文件 for quadtree
//
// 目标：覆盖 new/insert/search/duplicate/length 的基本语义。
//
// 注意：该 quadtree 实现的 `quadtree_search()` 仅对“已插入的点”安全；
// 对不存在的点进行 search 可能会递归到 NULL 子节点并崩溃（上游 C 测试也会避免这类查询）。

use crate::src_quadtree::*;
use core::ffi::c_void;

unsafe fn qt_length(qt: *mut crate::types::quadtree_t) -> u32 {
    // quadtree.h: `unsigned int length`
    *(crate::compat::c2r_field_ptr_quadtree_t__length(qt as *mut c_void) as *mut u32)
}

unsafe fn qt_root_nonnull(qt: *mut crate::types::quadtree_t) -> bool {
    let root_ptr = *(crate::compat::c2r_field_ptr_quadtree_t__root(qt as *mut c_void) as *mut *mut c_void);
    !root_ptr.is_null()
}

unsafe fn pt_x(p: *mut crate::types::quadtree_point_t) -> f64 {
    *(crate::compat::c2r_field_ptr_quadtree_point_t__x(p as *mut c_void) as *mut f64)
}

unsafe fn pt_y(p: *mut crate::types::quadtree_point_t) -> f64 {
    *(crate::compat::c2r_field_ptr_quadtree_point_t__y(p as *mut c_void) as *mut f64)
}

#[test]
fn test_quadtree_new_and_free() {
    unsafe {
        // 使用非整数边界，避免点落在 split line 上（node_contains_ 使用严格不等式）。
        let qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
        assert!(!qt.is_null(), "quadtree_new 应该返回有效指针");
        assert!(qt_root_nonnull(qt), "root 不应为 null");
        assert_eq!(qt_length(qt), 0u32, "新建树的 length 应为 0");
        quadtree_free(qt);
    }
}

#[test]
fn test_quadtree_insert_and_search() {
    unsafe {
        let qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
        assert!(!qt.is_null());

        let key = 0x1234usize as *mut c_void;
        let rc = quadtree_insert(qt, 10.0, 20.0, key);
        assert_eq!(rc, 1, "insert 应该成功");
        assert_eq!(qt_length(qt), 1u32, "length 应增加");

        let p = quadtree_search(qt, 10.0, 20.0);
        assert!(!p.is_null(), "search 应该能找到已插入点");
        assert_eq!(pt_x(p), 10.0);
        assert_eq!(pt_y(p), 20.0);

        quadtree_free(qt);
    }
}

#[test]
fn test_quadtree_duplicate_insert() {
    unsafe {
        let qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
        assert!(!qt.is_null());

        let key1 = 0x1111usize as *mut c_void;
        let key2 = 0x2222usize as *mut c_void;

        assert_eq!(quadtree_insert(qt, 1.0, 2.0, key1), 1);
        assert_eq!(qt_length(qt), 1u32);

        // 同坐标重复插入应失败，且不改变已有点
        assert_eq!(quadtree_insert(qt, 1.0, 2.0, key2), 0);
        assert_eq!(qt_length(qt), 1u32);

        let p = quadtree_search(qt, 1.0, 2.0);
        assert!(!p.is_null());
        assert_eq!(pt_x(p), 1.0);
        assert_eq!(pt_y(p), 2.0);

        quadtree_free(qt);
    }
}

#[test]
fn test_quadtree_multiple_points() {
    unsafe {
        let qt = quadtree_new(-1.0, -1.0, 1000.0, 1000.0);
        assert!(!qt.is_null());

        let a = 0xaaaausize as *mut c_void;
        let b = 0xbbbbusize as *mut c_void;
        let c = 0xccccusize as *mut c_void;

        assert_eq!(quadtree_insert(qt, 10.0, 10.0, a), 1);
        assert_eq!(quadtree_insert(qt, 90.0, 10.0, b), 1);
        assert_eq!(quadtree_insert(qt, 50.0, 90.0, c), 1);
        assert_eq!(qt_length(qt), 3u32);

        let pa = quadtree_search(qt, 10.0, 10.0);
        let pb = quadtree_search(qt, 90.0, 10.0);
        let pc = quadtree_search(qt, 50.0, 90.0);
        assert!(!pa.is_null() && !pb.is_null() && !pc.is_null());
        assert_eq!(pt_x(pa), 10.0);
        assert_eq!(pt_y(pa), 10.0);
        assert_eq!(pt_x(pb), 90.0);
        assert_eq!(pt_y(pb), 10.0);
        assert_eq!(pt_x(pc), 50.0);
        assert_eq!(pt_y(pc), 90.0);

        quadtree_free(qt);
    }
}
