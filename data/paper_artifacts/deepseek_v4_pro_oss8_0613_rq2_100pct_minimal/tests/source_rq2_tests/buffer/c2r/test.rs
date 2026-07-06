// C2R 框架测试文件 for buffer
// 直接调用翻译后的 Rust 函数
//
// 使用方法：
// 1. 复制此文件到项目的 src/test_c2r.rs
// 2. 在 main.rs 中添加: #[cfg(test)] mod test_c2r;
// 3. 运行 cargo test

use crate::src_buffer::*;
use crate::types::*;
use std::ffi::CString;

// 辅助函数：创建 C 字符串
fn make_cstr(s: &str) -> *mut ::core::ffi::c_char {
    CString::new(s).unwrap().into_raw()
}

// 辅助函数：释放 C 字符串
unsafe fn free_cstr(ptr: *mut ::core::ffi::c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

#[test]
fn test_buffer_new() {
    let buf = buffer_new();
    assert!(!buf.is_null(), "buffer_new 应该返回有效指针");
    buffer_free(buf);
}

#[test]
fn test_buffer_new_with_size() {
    let buf = buffer_new_with_size(64);
    assert!(!buf.is_null(), "buffer_new_with_size 应该返回有效指针");
    unsafe {
        assert!(buffer_size(buf) >= 64, "buffer 大小应该至少是请求的大小");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_new_with_string() {
    let s = make_cstr("hello");
    let buf = buffer_new_with_string(s);
    assert!(!buf.is_null(), "buffer_new_with_string 应该返回有效指针");
    unsafe {
        assert_eq!(buffer_length(buf), 5, "buffer 长度应该是 5");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_append() {
    let buf = buffer_new();
    let s = make_cstr("hello");
    unsafe {
        buffer_append(buf, s);
        assert_eq!(buffer_length(buf), 5, "追加后长度应该是 5");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_append_multiple() {
    let buf = buffer_new();
    let s1 = make_cstr("hello");
    let s2 = make_cstr(" world");
    unsafe {
        buffer_append(buf, s1);
        buffer_append(buf, s2);
        assert_eq!(buffer_length(buf), 11, "追加多个字符串后长度应该是 11");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_prepend() {
    let buf = buffer_new();
    let s1 = make_cstr("world");
    let s2 = make_cstr("hello ");
    unsafe {
        buffer_append(buf, s1);
        buffer_prepend(buf, s2);
        assert_eq!(buffer_length(buf), 11, "prepend 后长度应该是 11");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_equals() {
    let s = make_cstr("hello");
    let buf1 = buffer_new_with_string(s);
    let s2 = make_cstr("hello");
    let buf2 = buffer_new_with_string(s2);
    unsafe {
        assert_eq!(buffer_equals(buf1, buf2), 1, "相同内容的 buffer 应该相等");
    }
    buffer_free(buf1);
    buffer_free(buf2);
}

#[test]
fn test_buffer_trim() {
    let s = make_cstr("  hello  ");
    let buf = buffer_new_with_string(s);
    unsafe {
        buffer_trim(buf);
        assert_eq!(buffer_length(buf), 5, "trim 后长度应该是 5");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_trim_left() {
    let s = make_cstr("  hello");
    let buf = buffer_new_with_string(s);
    unsafe {
        buffer_trim_left(buf);
        assert_eq!(buffer_length(buf), 5, "trim_left 后长度应该是 5");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_trim_right() {
    let s = make_cstr("hello  ");
    let buf = buffer_new_with_string(s);
    unsafe {
        buffer_trim_right(buf);
        assert_eq!(buffer_length(buf), 5, "trim_right 后长度应该是 5");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_resize() {
    let buf = buffer_new_with_size(10);
    unsafe {
        buffer_resize(buf, 100);
        assert!(buffer_size(buf) >= 100, "resize 后大小应该至少是 100");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_slice() {
    let s = make_cstr("hello world");
    let buf = buffer_new_with_string(s);
    unsafe {
        let sliced = buffer_slice(buf, 0, 5);
        if !sliced.is_null() {
            assert_eq!(buffer_length(sliced), 5, "slice 后长度应该是 5");
            buffer_free(sliced);
        }
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_indexof() {
    let s = make_cstr("hello world");
    let buf = buffer_new_with_string(s);
    let needle = make_cstr("world");
    unsafe {
        let idx = buffer_indexof(buf, needle);
        assert_eq!(idx, 6, "world 在 hello world 中的索引应该是 6");
    }
    buffer_free(buf);
}

#[test]
fn test_buffer_indexof_not_found() {
    let s = make_cstr("hello");
    let buf = buffer_new_with_string(s);
    let needle = make_cstr("xyz");
    unsafe {
        let idx = buffer_indexof(buf, needle);
        assert_eq!(idx, -1, "找不到的字符串应该返回 -1");
    }
    buffer_free(buf);
}
