// c2rust 测试文件 for ht (hash table)
// 适配 c2rust 原始指针版本的函数签名
//
// c2rust 函数签名:
// - ht_create() -> *mut ht
// - ht_destroy(table: *mut ht)

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
fn test_ht_destroy_null() {
    unsafe {
        // 测试 destroy null 不会崩溃
        ht_destroy(std::ptr::null_mut());
    }
}

#[test]
#[ignore]
fn test_ht_compile_check() {
    // 验证代码可以编译
}
