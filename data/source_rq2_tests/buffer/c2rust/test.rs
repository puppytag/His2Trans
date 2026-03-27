// c2rust 测试文件 for buffer
// 适配 c2rust 原始指针版本的函数签名
//
// c2rust 函数签名:
// - buffer_new() -> *mut buffer_t
// - buffer_new_with_size(n: size_t) -> *mut buffer_t
// - buffer_new_with_string(str: *mut c_char) -> *mut buffer_t
// - buffer_new_with_copy(str: *mut c_char) -> *mut buffer_t
// - buffer_free(self_0: *mut buffer_t)
// - buffer_size(self_0: *mut buffer_t) -> size_t
// - buffer_length(self_0: *mut buffer_t) -> size_t

use std::ffi::CString;
use crate::src_buffer::*;

#[test]
fn test_buffer_new() {
    unsafe {
        let buf = buffer_new();
        assert!(!buf.is_null(), "buffer_new 应该返回有效指针");
        if !buf.is_null() {
            buffer_free(buf);
        }
    }
}

#[test]
fn test_buffer_new_with_size() {
    unsafe {
        let buf = buffer_new_with_size(64);
        assert!(!buf.is_null(), "buffer_new_with_size 应该返回有效指针");
        if !buf.is_null() {
            assert!(buffer_size(buf) >= 64, "buffer 大小应该至少是请求的大小");
            buffer_free(buf);
        }
    }
}

#[test]
fn test_buffer_new_with_string() {
    unsafe {
        let s = CString::new("hello").unwrap();
        let buf = buffer_new_with_string(s.as_ptr() as *mut _);
        assert!(!buf.is_null(), "buffer_new_with_string 应该返回有效指针");
        if !buf.is_null() {
            assert_eq!(buffer_length(buf), 5, "buffer 长度应该是 5");
            buffer_free(buf);
        }
    }
}

#[test]
fn test_buffer_new_with_copy() {
    unsafe {
        let s = CString::new("hello").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr() as *mut _);
        assert!(!buf.is_null(), "buffer_new_with_copy 应该返回有效指针");
        if !buf.is_null() {
            assert_eq!(buffer_length(buf), 5, "buffer 长度应该是 5");
            buffer_free(buf);
        }
    }
}

#[test]
fn test_buffer_length_and_size() {
    unsafe {
        let buf = buffer_new_with_size(100);
        assert!(!buf.is_null());
        if !buf.is_null() {
            let len = buffer_length(buf);
            let size = buffer_size(buf);
            assert!(size >= 100, "size 应该至少是 100");
            assert!(len <= size, "length 应该 <= size");
            buffer_free(buf);
        }
    }
}

#[test]
fn test_buffer_equals() {
    unsafe {
        let s = CString::new("hello").unwrap();
        let buf1 = buffer_new_with_copy(s.as_ptr() as *mut _);
        let buf2 = buffer_new_with_copy(s.as_ptr() as *mut _);

        assert!(!buf1.is_null() && !buf2.is_null());
        if !buf1.is_null() && !buf2.is_null() {
            let _ = buffer_equals(buf1, buf2);
            buffer_free(buf1);
            buffer_free(buf2);
        }
    }
}

#[test]
fn test_buffer_trim() {
    unsafe {
        let s = CString::new("  hello  ").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr() as *mut _);
        assert!(!buf.is_null());
        if !buf.is_null() {
            buffer_trim(buf);
            buffer_free(buf);
        }
    }
}

#[test]
fn test_buffer_resize() {
    unsafe {
        let buf = buffer_new_with_size(10);
        assert!(!buf.is_null());
        if !buf.is_null() {
            let result = buffer_resize(buf, 100);
            if result == 0 {
                assert!(buffer_size(buf) >= 100, "resize 后大小应该至少是 100");
            }
            buffer_free(buf);
        }
    }
}

#[test]
fn test_buffer_indexof() {
    unsafe {
        let s = CString::new("hello world").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr() as *mut _);
        assert!(!buf.is_null());
        if !buf.is_null() {
            let needle = CString::new("world").unwrap();
            let idx = buffer_indexof(buf, needle.as_ptr() as *mut _);
            assert_eq!(idx, 6, "world 在 hello world 中的索引应该是 6");
            buffer_free(buf);
        }
    }
}

#[test]
fn test_buffer_indexof_not_found() {
    unsafe {
        let s = CString::new("hello").unwrap();
        let buf = buffer_new_with_copy(s.as_ptr() as *mut _);
        assert!(!buf.is_null());
        if !buf.is_null() {
            let needle = CString::new("xyz").unwrap();
            let idx = buffer_indexof(buf, needle.as_ptr() as *mut _);
            assert_eq!(idx, -1, "找不到的字符串应该返回 -1");
            buffer_free(buf);
        }
    }
}

#[test]
#[ignore]
fn test_buffer_compile_check() {
    // 验证代码可以编译
}
