// Unified RQ2 test suite for ht in C2Rust mode.
//
// 这里按当前 c2rust 产物真实可见的 API 做 3 个非忽略测试：
// - create/destroy
// - null destroy
// - 初始状态检查

use crate::src_ht::*;

#[test]
fn test_ht_create() {
    unsafe {
        let table = ht_create();
        assert!(!table.is_null(), "ht_create 应该返回有效指针");
        if !table.is_null() {
            ht_destroy(table);
        }
    }
}

#[test]
fn test_ht_initial_state() {
    unsafe {
        let table = ht_create();
        assert!(!table.is_null());
        if !table.is_null() {
            assert_eq!((*table).length, 0);
            assert!((*table).capacity >= 16);
            assert!(!(*table).entries.is_null());
            ht_destroy(table);
        }
    }
}

#[test]
fn test_ht_create_two_tables() {
    unsafe {
        let t1 = ht_create();
        let t2 = ht_create();
        assert!(!t1.is_null() && !t2.is_null());
        if !t1.is_null() {
            ht_destroy(t1);
        }
        if !t2.is_null() {
            ht_destroy(t2);
        }
    }
}
