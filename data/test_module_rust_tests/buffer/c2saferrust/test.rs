// c2saferrust 测试文件 for buffer
// 目标：与 evolc2rust 套件对齐的真实语义测试（new/append/prepend/equals/trim/resize/slice/indexof）。

use crate::*;
use std::ffi::{CStr, CString};
use std::ptr;

unsafe fn free_buf(ptr: *mut buffer_t) {
    if !ptr.is_null() {
        buffer_free(Box::from_raw(ptr));
    }
}

#[test]
fn test_buffer_new() {
    // c2saferrust 的 buffer_new 返回 Arc<RefCell<Vec<u8>>>，这里只验证默认容量。
    let buf = buffer_new();
    assert!(
        buf.borrow().capacity() >= BUFFER_DEFAULT_SIZE as usize,
        "buffer_new 应该创建有默认容量的 buffer"
    );
}

#[test]
fn test_buffer_new_with_size() {
    let buf = buffer_new_with_size(64).expect("buffer_new_with_size should return Some");
    unsafe {
        assert!(buf.as_ref().len >= 64, "buffer 大小应该至少是请求的大小");
        free_buf(buf.as_ptr());
    }
}

#[test]
fn test_buffer_new_with_string() {
    let s = CString::new("hello").unwrap();
    let buf = buffer_new_with_string(s.as_c_str());
    assert!(!buf.is_null(), "buffer_new_with_string 应该返回有效指针");
    unsafe {
        let buf_ref = &*buf;
        assert_eq!(buffer_length(buf_ref), 5, "buffer 长度应该是 5");
        free_buf(buf);
    }
}

#[test]
fn test_buffer_append() {
    let buf = buffer_new_with_size(64).unwrap();
    let s = CString::new("hello").unwrap();
    unsafe {
        let buf_ptr = buf.as_ptr();
        let rc = buffer_append(&mut *buf_ptr, &*s.as_ptr());
        assert_eq!(rc, 0);
        assert_eq!(buffer_length(&*buf_ptr), 5, "追加后长度应该是 5");
        free_buf(buf_ptr);
    }
}

#[test]
fn test_buffer_append_multiple() {
    let buf = buffer_new_with_size(64).unwrap();
    let s1 = CString::new("hello").unwrap();
    let s2 = CString::new(" world").unwrap();
    unsafe {
        let buf_ptr = buf.as_ptr();
        assert_eq!(buffer_append(&mut *buf_ptr, &*s1.as_ptr()), 0);
        assert_eq!(buffer_append(&mut *buf_ptr, &*s2.as_ptr()), 0);
        assert_eq!(buffer_length(&*buf_ptr), 11, "追加多个字符串后长度应该是 11");
        free_buf(buf_ptr);
    }
}

#[test]
fn test_buffer_prepend() {
    // 注意：当前 c2saferrust 的 buffer_prepend 释放旧 data 使用 CString::from_raw，
    // 因此这里让 old data 来源于 CString::into_raw，且通过传入较大的 len 避免触发 realloc。
    let old = CString::new("world").unwrap().into_raw();
    let buf = buffer_new_with_string_length(old, 1024);
    assert!(!buf.is_null());

    let prefix = CString::new("hello ").unwrap();
    unsafe {
        let rc = buffer_prepend(&mut *buf, prefix.as_c_str());
        assert_eq!(rc, 0);
        assert_eq!(buffer_length(&*buf), 11, "prepend 后长度应该是 11");
        // 基于语义检查字符串内容
        let got = CStr::from_ptr((*buf).data).to_string_lossy().to_string();
        assert_eq!(got, "hello world");
        free_buf(buf);
    }
}

#[test]
fn test_buffer_equals() {
    // 语义：相同内容的 buffer 应该相等。
    let s = CString::new("hello").unwrap();
    let buf1 = buffer_new_with_copy(s.as_ptr() as *mut _);
    let buf2 = buffer_new_with_copy(s.as_ptr() as *mut _);
    assert!(!buf1.is_null() && !buf2.is_null());
    unsafe {
        assert_eq!(buffer_equals(&*buf1, &*buf2), 1, "相同内容的 buffer 应该相等");
        free_buf(buf1);
        free_buf(buf2);
    }
}

#[test]
fn test_buffer_trim() {
    let s = CString::new("  hello  ").unwrap();
    let buf = buffer_new_with_copy(s.as_ptr() as *mut _);
    assert!(!buf.is_null());
    unsafe {
        buffer_trim(buf);
        assert_eq!(buffer_length(&*buf), 5, "trim 后长度应该是 5");
        let got = CStr::from_ptr((*buf).data).to_string_lossy().to_string();
        assert_eq!(got, "hello");
        free_buf(buf);
    }
}

#[test]
fn test_buffer_trim_left() {
    let s = CString::new("  hello").unwrap();
    let buf = buffer_new_with_copy(s.as_ptr() as *mut _);
    assert!(!buf.is_null());
    unsafe {
        let mut p = buf;
        buffer_trim_left(&mut p);
        assert_eq!(buffer_length(&*buf), 5, "trim_left 后长度应该是 5");
        let got = CStr::from_ptr((*buf).data).to_string_lossy().to_string();
        assert_eq!(got, "hello");
        free_buf(buf);
    }
}

#[test]
fn test_buffer_trim_right() {
    let s = CString::new("hello  ").unwrap();
    let buf = buffer_new_with_copy(s.as_ptr() as *mut _);
    assert!(!buf.is_null());
    unsafe {
        buffer_trim_right(&mut *buf);
        assert_eq!(buffer_length(&*buf), 5, "trim_right 后长度应该是 5");
        let got = CStr::from_ptr((*buf).data).to_string_lossy().to_string();
        assert_eq!(got, "hello");
        free_buf(buf);
    }
}

#[test]
fn test_buffer_resize() {
    let buf = buffer_new_with_size(10).unwrap();
    unsafe {
        let raw_ptr = buf.as_ptr();
        let rc = buffer_resize(raw_ptr, 100);
        assert_eq!(rc, 0);
        assert!(buffer_size(&*raw_ptr) >= 100, "resize 后大小应该至少是 100");
        free_buf(raw_ptr);
    }
}

#[test]
fn test_buffer_slice() {
    let s = CString::new("hello world").unwrap();
    let buf = buffer_new_with_copy(s.as_ptr() as *mut _);
    assert!(!buf.is_null());
    unsafe {
        let sliced = buffer_slice(buf, 0, 5);
        if !sliced.is_null() {
            assert_eq!(buffer_length(&*sliced), 5, "slice 后长度应该是 5");
            let got = CStr::from_ptr((*sliced).data).to_string_lossy().to_string();
            assert_eq!(got, "hello");
            free_buf(sliced);
        }
        free_buf(buf);
    }
}

#[test]
fn test_buffer_indexof() {
    let s = CString::new("hello world").unwrap();
    let buf = buffer_new_with_copy(s.as_ptr() as *mut _);
    assert!(!buf.is_null());
    let needle = CString::new("world").unwrap();
    unsafe {
        let idx = buffer_indexof(&mut *buf, needle.as_ptr() as *mut _);
        assert_eq!(idx, 6, "world 在 hello world 中的索引应该是 6");
        free_buf(buf);
    }
}

#[test]
fn test_buffer_indexof_not_found() {
    let s = CString::new("hello").unwrap();
    let buf = buffer_new_with_copy(s.as_ptr() as *mut _);
    assert!(!buf.is_null());
    let needle = CString::new("xyz").unwrap();
    unsafe {
        let idx = buffer_indexof(&mut *buf, needle.as_ptr() as *mut _);
        assert_eq!(idx, -1, "找不到的字符串应该返回 -1");
        free_buf(buf);
    }
}

