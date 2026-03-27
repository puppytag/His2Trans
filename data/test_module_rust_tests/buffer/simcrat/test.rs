// simcrat 测试文件 for buffer
//
// 目标：与 evolc2rust/c2saferrust 的外部测试语义保持一致（14 个 #[test]）。
// 注意：simcrat 的 buffer 实现如果存在类型/内存问题，这些测试会真实失败或崩溃。

use crate::*;

fn as_bytes(s: &str) -> &[u8] {
    s.as_bytes()
}

#[test]
fn test_buffer_new() {
    let buf = buffer_new().expect("buffer_new failed");
    drop(buf);
}

#[test]
fn test_buffer_new_with_size() {
    let buf = buffer_new_with_size(64).expect("buffer_new_with_size failed");
    // 只验证不会崩溃（size/len 的具体含义依赖实现）
    let _ = buffer_size(*buf);
    drop(buf);
}

#[test]
fn test_buffer_new_with_string() {
    let buf = buffer_new_with_string("hello").expect("buffer_new_with_string failed");
    assert_eq!(buffer_length(*buf), 5);
    drop(buf);
}

#[test]
fn test_buffer_append() {
    let mut buf = buffer_new().expect("buffer_new failed");
    buffer_append(&mut *buf, as_bytes("hello")).expect("buffer_append failed");
    assert_eq!(buffer_length(*buf), 5);
}

#[test]
fn test_buffer_append_multiple() {
    let mut buf = buffer_new().expect("buffer_new failed");
    buffer_append(&mut *buf, as_bytes("hello")).expect("buffer_append failed");
    buffer_append(&mut *buf, as_bytes(" world")).expect("buffer_append failed");
    assert_eq!(buffer_length(*buf), 11);
}

#[test]
fn test_buffer_prepend() {
    // simcrat 的 buffer_prepend 以裸指针形式工作，这里尽量按“预期用法”调用。
    let mut buf = buffer_new_with_size(64).expect("buffer_new_with_size failed");
    buffer_append(&mut *buf, as_bytes("world")).expect("buffer_append failed");
    let buf_ptr = *buf as *mut u8;
    buffer_prepend(buf_ptr, "hello ").expect("buffer_prepend failed");
    assert_eq!(buffer_length(*buf), 11);
}

#[test]
fn test_buffer_equals() {
    let buf1 = buffer_new_with_string("hello").expect("buffer_new_with_string failed");
    let buf2 = buffer_new_with_string("hello").expect("buffer_new_with_string failed");
    assert!(buffer_equals(*buf1, &*buf2));
}

#[test]
fn test_buffer_trim() {
    let mut buf = buffer_new_with_string("  hello  ").expect("buffer_new_with_string failed");
    buffer_trim(&mut *buf);
    assert_eq!(buffer_length(*buf), 5);
}

#[test]
fn test_buffer_trim_left() {
    let mut buf = buffer_new_with_string("  hello").expect("buffer_new_with_string failed");
    let p: *mut BufferT = &mut *buf;
    buffer_trim_left(p);
    assert_eq!(buffer_length(*buf), 5);
}

#[test]
fn test_buffer_trim_right() {
    let mut buf = buffer_new_with_string("hello  ").expect("buffer_new_with_string failed");
    let mut raw = *buf;
    buffer_trim_right(&mut raw);
    assert_eq!(buffer_length(raw), 5);
}

#[test]
fn test_buffer_resize() {
    let buf = buffer_new_with_size(10).expect("buffer_new_with_size failed");
    buffer_resize(*buf, 100).expect("buffer_resize failed");
    let _ = buffer_size(*buf);
}

#[test]
fn test_buffer_slice() {
    let buf = buffer_new_with_string("hello world").expect("buffer_new_with_string failed");
    let sliced = buffer_slice(&*buf, 0, 5).expect("buffer_slice failed");
    assert_eq!(buffer_length(*sliced), 5);
}

#[test]
fn test_buffer_indexof() {
    let buf = buffer_new_with_string("hello world").expect("buffer_new_with_string failed");
    let idx = buffer_indexof(*buf, as_bytes("world"));
    assert_eq!(idx, Some(6));
}

#[test]
fn test_buffer_indexof_not_found() {
    let buf = buffer_new_with_string("hello").expect("buffer_new_with_string failed");
    let idx = buffer_indexof(*buf, as_bytes("xyz"));
    assert_eq!(idx, None);
}

