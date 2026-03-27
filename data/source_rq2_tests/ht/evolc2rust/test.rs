// evo-c2rust-v2 测试文件 for ht (hash table)
// 目标：真实功能测试（set/get/has/del/size/clear + 迭代接口）。
//
// 注意：只使用公开的 `#[no_mangle] pub unsafe extern "C"` API，避免访问私有 helper。

use crate::src::ht_c::*;

use libc::{c_char, c_void};
use std::ffi::CString;

#[test]
fn test_hash_new_and_free() {
    unsafe {
        let h = hash_new();
        assert!(!h.is_null(), "hash_new 应该返回有效指针");
        hash_free(h);
    }
}

#[test]
fn test_hash_set_get_has_del_size_clear() {
    unsafe {
        let h = hash_new();
        assert!(!h.is_null());

        let k1 = CString::new("k1").unwrap();
        let k2 = CString::new("k2").unwrap();

        let v1 = 0x1111usize as *mut c_void;
        let v2 = 0x2222usize as *mut c_void;

        assert_eq!(hash_size(h), 0);
        assert_eq!(hash_has(h, k1.as_ptr()), 0);
        assert!(hash_get(h, k1.as_ptr()).is_null());

        hash_set(h, k1.as_ptr(), v1);
        assert_eq!(hash_size(h), 1);
        assert_eq!(hash_has(h, k1.as_ptr()), 1);
        assert_eq!(hash_get(h, k1.as_ptr()), v1);

        // 覆盖写入同一个 key：size 不变，value 更新
        hash_set(h, k1.as_ptr(), v2);
        assert_eq!(hash_size(h), 1);
        assert_eq!(hash_get(h, k1.as_ptr()), v2);

        hash_set(h, k2.as_ptr(), v1);
        assert_eq!(hash_size(h), 2);
        assert_eq!(hash_has(h, k2.as_ptr()), 1);
        assert_eq!(hash_get(h, k2.as_ptr()), v1);

        hash_del(h, k1.as_ptr());
        assert_eq!(hash_size(h), 1);
        assert_eq!(hash_has(h, k1.as_ptr()), 0);
        assert!(hash_get(h, k1.as_ptr()).is_null());

        hash_clear(h);
        assert_eq!(hash_size(h), 0);
        assert_eq!(hash_has(h, k2.as_ptr()), 0);
        assert!(hash_get(h, k2.as_ptr()).is_null());

        hash_free(h);
    }
}

#[test]
fn test_kh_iteration_api() {
    unsafe {
        let h = hash_new();
        assert!(!h.is_null());

        let k1 = CString::new("apple").unwrap();
        let k2 = CString::new("banana").unwrap();
        let v1 = 0xaaaausize as *mut c_void;
        let v2 = 0xbbbbusize as *mut c_void;
        hash_set(h, k1.as_ptr(), v1);
        hash_set(h, k2.as_ptr(), v2);

        let mut found = 0;
        let mut i = kh_begin(h);
        let end = kh_end(h);
        while i != end {
            if kh_exist(h, i) != 0 {
                let key_ptr = kh_key(h, i);
                assert!(!key_ptr.is_null());
                let key = std::ffi::CStr::from_ptr(key_ptr as *const c_char)
                    .to_string_lossy()
                    .to_string();
                let val = kh_value(h, i);
                if key == "apple" {
                    assert_eq!(val, v1);
                    found += 1;
                } else if key == "banana" {
                    assert_eq!(val, v2);
                    found += 1;
                } else {
                    panic!("unexpected key from iterator: {key}");
                }
            }
            i += 1;
        }

        assert_eq!(found, 2, "迭代接口应该能遍历到所有元素");
        hash_free(h);
    }
}

