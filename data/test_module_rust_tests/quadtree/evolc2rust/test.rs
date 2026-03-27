// evo-c2rust-v2 测试文件 for quadtree
// 目标：真实功能测试（new/insert/search/duplicate/length）。
//
// 注意：当前 evo-c2rust-v2 输出的 `src/mod.rs` 仅导出 `quadtree_c`，
// 因此测试也只针对 `quadtree_c` 中可用的公开 API。

use crate::src::quadtree_c::*;

use libc::c_void;

#[test]
fn test_quadtree_new_and_free() {
    unsafe {
        let qt = quadtree_new(0.0, 0.0, 100.0, 100.0);
        assert!(!qt.is_null(), "quadtree_new 应该返回有效指针");
        assert!(!(*qt).root.is_null(), "root 不应为 null");
        assert_eq!((*qt).length, 0, "新建树的 length 应为 0");
        quadtree_free(qt);
    }
}

#[test]
fn test_quadtree_insert_and_search() {
    unsafe {
        let qt = quadtree_new(0.0, 0.0, 100.0, 100.0);
        assert!(!qt.is_null());

        let key = 0x1234usize as *mut c_void;
        let rc = quadtree_insert(qt, 10.0, 20.0, key);
        assert_eq!(rc, 1, "insert 应该成功");
        assert_eq!((*qt).length, 1, "length 应增加");

        let p = quadtree_search(qt, 10.0, 20.0);
        assert!(!p.is_null(), "search 应该能找到已插入点");
        assert_eq!((*p).x, 10.0);
        assert_eq!((*p).y, 20.0);

        let miss = quadtree_search(qt, 11.0, 20.0);
        assert!(miss.is_null(), "不存在的点应该返回 null");

        quadtree_free(qt);
    }
}

#[test]
fn test_quadtree_duplicate_insert() {
    unsafe {
        let qt = quadtree_new(0.0, 0.0, 100.0, 100.0);
        assert!(!qt.is_null());

        let key1 = 0x1111usize as *mut c_void;
        let key2 = 0x2222usize as *mut c_void;

        assert_eq!(quadtree_insert(qt, 1.0, 2.0, key1), 1);
        assert_eq!((*qt).length, 1);

        // 同坐标重复插入应失败，且不改变已有点
        assert_eq!(quadtree_insert(qt, 1.0, 2.0, key2), 0);
        assert_eq!((*qt).length, 1);

        let p = quadtree_search(qt, 1.0, 2.0);
        assert!(!p.is_null());
        assert_eq!((*p).x, 1.0);
        assert_eq!((*p).y, 2.0);

        quadtree_free(qt);
    }
}

#[test]
fn test_quadtree_multiple_points() {
    unsafe {
        let qt = quadtree_new(0.0, 0.0, 100.0, 100.0);
        assert!(!qt.is_null());

        let a = 0xaaaausize as *mut c_void;
        let b = 0xbbbbusize as *mut c_void;
        let c = 0xccccusize as *mut c_void;

        assert_eq!(quadtree_insert(qt, 10.0, 10.0, a), 1);
        assert_eq!(quadtree_insert(qt, 90.0, 10.0, b), 1);
        assert_eq!(quadtree_insert(qt, 50.0, 90.0, c), 1);
        assert_eq!((*qt).length, 3);

        let pa = quadtree_search(qt, 10.0, 10.0);
        let pb = quadtree_search(qt, 90.0, 10.0);
        let pc = quadtree_search(qt, 50.0, 90.0);
        assert!(!pa.is_null() && !pb.is_null() && !pc.is_null());
        assert_eq!((*pa).x, 10.0);
        assert_eq!((*pa).y, 10.0);
        assert_eq!((*pb).x, 90.0);
        assert_eq!((*pb).y, 10.0);
        assert_eq!((*pc).x, 50.0);
        assert_eq!((*pc).y, 90.0);

        quadtree_free(qt);
    }
}
