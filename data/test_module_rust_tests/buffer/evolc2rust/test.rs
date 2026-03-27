// evo-c2rust-v2 测试文件 for buffer
// 直接调用 Rust 函数而不是通过 FFI

use crate::translation_utils::*;
use crate::src::buffer_c::*;
use crate::src::buffer_h::*;

// 辅助函数：创建 C 字符串
fn make_cstr(s: &str) -> Ptr<u8> {
    let bytes = s.as_bytes();
    let mut ptr: Ptr<u8> = c_malloc!(bytes.len() + 1);
    for i in 0..bytes.len() {
        ptr[i] = bytes[i];
    }
    ptr[bytes.len()] = 0;
    ptr
}

#[test]
fn test_buffer_new() {
    let buf = buffer_new();
    assert!(buf.as_bool(), "buffer_new 应该返回有效指针");
    buffer_free(buf);
}

#[test]
fn test_buffer_new_with_size() {
    let buf = buffer_new_with_size(64);
    assert!(buf.as_bool(), "buffer_new_with_size 应该返回有效指针");
    assert!(buffer_size(buf) >= 64, "buffer 大小应该至少是请求的大小");
    buffer_free(buf);
}

#[test]
fn test_buffer_new_with_string() {
    let mut s = make_cstr("hello");
    let buf = buffer_new_with_string(s);
    assert!(buf.as_bool(), "buffer_new_with_string 应该返回有效指针");
    assert_eq!(buffer_length(buf), 5, "buffer 长度应该是 5");
    buffer_free(buf);
    c_free!(s);
}

#[test]
fn test_buffer_append() {
    let buf = buffer_new();
    let mut s = make_cstr("hello");
    buffer_append(buf, s);
    assert_eq!(buffer_length(buf), 5, "追加后长度应该是 5");
    buffer_free(buf);
    c_free!(s);
}

#[test]
fn test_buffer_append_multiple() {
    let buf = buffer_new();
    let mut s1 = make_cstr("hello");
    let mut s2 = make_cstr(" world");
    buffer_append(buf, s1);
    buffer_append(buf, s2);
    assert_eq!(buffer_length(buf), 11, "追加多个字符串后长度应该是 11");
    buffer_free(buf);
    c_free!(s1);
    c_free!(s2);
}

#[test]
fn test_buffer_prepend() {
    let buf = buffer_new();
    let mut s1 = make_cstr("world");
    let mut s2 = make_cstr("hello ");
    buffer_append(buf, s1);
    buffer_prepend(buf, s2);
    assert_eq!(buffer_length(buf), 11, "prepend 后长度应该是 11");
    buffer_free(buf);
    c_free!(s1);
    c_free!(s2);
}

#[test]
fn test_buffer_equals() {
    let mut s = make_cstr("hello");
    let buf1 = buffer_new_with_string(s);
    let buf2 = buffer_new_with_string(s);
    assert_eq!(buffer_equals(buf1, buf2), 1, "相同内容的 buffer 应该相等");
    buffer_free(buf1);
    buffer_free(buf2);
    c_free!(s);
}

#[test]
fn test_buffer_trim() {
    let mut s = make_cstr("  hello  ");
    let buf = buffer_new_with_string(s);
    buffer_trim(buf);
    assert_eq!(buffer_length(buf), 5, "trim 后长度应该是 5");
    buffer_free(buf);
    c_free!(s);
}

#[test]
fn test_buffer_trim_left() {
    let mut s = make_cstr("  hello");
    let buf = buffer_new_with_string(s);
    buffer_trim_left(buf);
    assert_eq!(buffer_length(buf), 5, "trim_left 后长度应该是 5");
    buffer_free(buf);
    c_free!(s);
}

#[test]
fn test_buffer_trim_right() {
    let mut s = make_cstr("hello  ");
    let buf = buffer_new_with_string(s);
    buffer_trim_right(buf);
    assert_eq!(buffer_length(buf), 5, "trim_right 后长度应该是 5");
    buffer_free(buf);
    c_free!(s);
}

#[test]
fn test_buffer_resize() {
    let buf = buffer_new_with_size(10);
    buffer_resize(buf, 100);
    assert!(buffer_size(buf) >= 100, "resize 后大小应该至少是 100");
    buffer_free(buf);
}

#[test]
fn test_buffer_slice() {
    let mut s = make_cstr("hello world");
    let buf = buffer_new_with_string(s);
    let sliced = buffer_slice(buf, 0, 5);
    if sliced.as_bool() {
        assert_eq!(buffer_length(sliced), 5, "slice 后长度应该是 5");
        buffer_free(sliced);
    }
    buffer_free(buf);
    c_free!(s);
}

#[test]
fn test_buffer_indexof() {
    let mut s = make_cstr("hello world");
    let buf = buffer_new_with_string(s);
    let mut needle = make_cstr("world");
    let idx = buffer_indexof(buf, needle);
    assert_eq!(idx, 6, "world 在 hello world 中的索引应该是 6");
    buffer_free(buf);
    c_free!(s);
    c_free!(needle);
}

#[test]
fn test_buffer_indexof_not_found() {
    let mut s = make_cstr("hello");
    let buf = buffer_new_with_string(s);
    let mut needle = make_cstr("xyz");
    let idx = buffer_indexof(buf, needle);
    assert_eq!(idx, -1, "找不到的字符串应该返回 -1");
    buffer_free(buf);
    c_free!(s);
    c_free!(needle);
}
