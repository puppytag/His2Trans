//! Module: src_buffer
//!
//! Safe Rust reimplementation of the dynamic string buffer (buffer_t).
//!
//! All public non-FFI functions accept `&buffer_t` / `&mut buffer_t`.
//! Only the three `extern "C"` shims and constructors/destructors that must
//! return/take `*mut buffer_t` keep raw pointer signatures.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;

use std::ffi::CStr;

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn nearest_multiple(n: usize) -> usize {
    (n + 1023) & !1023
}


/// Return the length of the null-terminated C string currently stored in
/// `b`.  Returns 0 for an empty or invalid data pointer.
fn content_len(b: &buffer_t) -> usize {
    if b.data_offset >= b.storage.len() {
        return 0;
    }
    let slice = &b.storage[b.data_offset..];
    slice.iter().position(|&c| c == 0).unwrap_or(slice.len())
}

// ---------------------------------------------------------------------------
// constructors (return *mut buffer_t)
// ---------------------------------------------------------------------------

/// Internal helper: allocate a buffer_t on the heap and return a Box.
fn buffer_new_with_size_box(n: size_t) -> Box<buffer_t> {
    let n = n as usize;
    Box::new(buffer_t {
        len: n,
        storage: vec![0u8; n + 1],
        data_offset: 0,
    })
}

pub fn buffer_new() -> *mut buffer_t {
    buffer_new_with_size(64)
}

pub fn buffer_new_with_size(n: size_t) -> *mut buffer_t {
    Box::into_raw(buffer_new_with_size_box(n))
}

pub fn buffer_new_with_string(data: &[u8]) -> *mut buffer_t {
    buffer_new_with_slice(data)
}

pub fn buffer_new_with_string_length(data: &[u8], _len: size_t) -> *mut buffer_t {
    // _len kept for backward compatibility; actual length is data.len().
    buffer_new_with_slice(data)
}

pub fn buffer_new_with_copy(data: &[u8]) -> *mut buffer_t {
    buffer_new_with_slice(data)
}

/// Safe constructor: allocate and return a new buffer_t filled with `data`.
fn buffer_new_with_slice(data: &[u8]) -> *mut buffer_t {
    let len = data.len();
    let mut b = buffer_new_with_size_box(len as size_t);
    if len > 0 {
        b.storage[..len].copy_from_slice(data);
    }
    Box::into_raw(b)
}

// ---------------------------------------------------------------------------
// queries
// ---------------------------------------------------------------------------

pub fn buffer_size(b: &buffer_t) -> size_t {
    b.len
}

pub fn buffer_length(b: &buffer_t) -> size_t {
    content_len(b) as size_t
}

// ---------------------------------------------------------------------------
// resize / compact / free
// ---------------------------------------------------------------------------

pub fn buffer_resize(b: &mut buffer_t, n: size_t) -> c_int {
    let n = nearest_multiple(n as usize);
    b.storage.resize(n + 1, 0u8);
    // C's realloc resets data to alloc base; we mirror that by resetting
    // data_offset to 0 (trim_left effects are lost across resize).
    b.len = n as size_t;
    b.data_offset = 0;
    0
}

pub fn buffer_compact(b: &mut buffer_t) -> ssize_t {
    let cur_len = content_len(b);
    let rem = (b.len as ssize_t) - (cur_len as ssize_t);
    if cur_len == b.len as usize {
        return rem; // already compact
    }
    // Allocate a new minimal block and copy only the live string.
    let mut new_storage = vec![0u8; cur_len + 1];
    if cur_len > 0 {
        let src = &b.storage[b.data_offset..][..cur_len];
        new_storage[..cur_len].copy_from_slice(src);
    }
    b.storage = new_storage;
    b.len = cur_len as size_t;
    b.data_offset = 0;
    rem
}

pub fn buffer_free(ptr: *mut buffer_t) {
    if ptr.is_null() {
        return;
    }
    // The Box drop will automatically free both the buffer_t struct and its
    // owned Vec<u8> storage.
    unsafe { drop(Box::from_raw(ptr)); }
}

// ---------------------------------------------------------------------------
// variadic appendf (implemented in C via shims)
// ---------------------------------------------------------------------------

#[allow(improper_ctypes)]
extern "C" {
    pub fn buffer_appendf(self_: *mut buffer_t, format: *const c_char, ...) -> c_int;
}

// ---------------------------------------------------------------------------
// append
// ---------------------------------------------------------------------------

pub fn buffer_append(b: &mut buffer_t, data: &[u8]) -> c_int {
    buffer_append_n(b, data, data.len() as size_t)
}

pub fn buffer_append_n(
    b: &mut buffer_t,
    data: &[u8],
    _len: size_t,
) -> c_int {
    let len = data.len();
    let prev = content_len(b);
    let needed = len + prev;

    // If not enough room, resize first.
    if (b.len as usize) <= needed {
        if buffer_resize(b, needed as size_t) == -1 {
            return -1;
        }
    }

    let start = b.data_offset + prev;
    b.storage[start..start + len].copy_from_slice(data);
    b.storage[start + len] = 0u8; // NUL
    0
}

pub fn buffer_prepend(b: &mut buffer_t, data: &[u8]) -> c_int {
    let len = data.len();
    let prev = content_len(b);
    let needed = len + prev;

    if (b.len as usize) <= needed {
        if buffer_resize(b, needed as size_t) == -1 {
            return -1;
        }
    }

    // Move existing string right by `len` bytes (including NUL).
    b.storage
        .copy_within(b.data_offset..b.data_offset + prev + 1, b.data_offset + len);

    b.storage[b.data_offset..b.data_offset + len].copy_from_slice(data);

    0
}

pub fn buffer_slice(
    b: &buffer_t,
    from: size_t,
    to: ssize_t,
) -> *mut buffer_t {
    let str_len = content_len(b) as isize;
    let mut to = to;
    if to < 0 {
        to = str_len - (!to);
    }
    let from = from as usize;
    let to = to as usize;
    if to < from || from > str_len as usize {
        return std::ptr::null_mut();
    }
    let to = to.min(str_len as usize);
    let n = to - from;

    let mut nb = buffer_new_with_size_box(n as size_t);
    if n > 0 {
        let src = &b.storage[b.data_offset + from..b.data_offset + from + n];
        nb.storage[..n].copy_from_slice(src);
        nb.storage[n] = 0u8;
    }
    Box::into_raw(nb)
}

// ---------------------------------------------------------------------------
// comparison / search
// ---------------------------------------------------------------------------

pub fn buffer_equals(a: &buffer_t, b: &buffer_t) -> c_int {
    let a_len = content_len(a);
    let b_len = content_len(b);
    if a_len != b_len {
        return 0;
    }
    let a_slice = &a.storage[a.data_offset..][..a_len];
    let b_slice = &b.storage[b.data_offset..][..b_len];
    if a_slice == b_slice { 1 } else { 0 }
}

pub fn buffer_indexof(b: &buffer_t, needle: &[u8]) -> ssize_t {
    if b.storage.is_empty() || needle.is_empty() {
        return -1;
    }
    let data_len = content_len(b);
    let haystack = &b.storage[b.data_offset..][..data_len];
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|p| p as ssize_t)
        .unwrap_or(-1)
}

// ---------------------------------------------------------------------------
// trim / fill / print
// ---------------------------------------------------------------------------

fn is_trim_char(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C)
}

pub fn buffer_trim_left(b: &mut buffer_t) {
    let data_len = content_len(b);
    if data_len == 0 { return; }

    let bytes = &b.storage[b.data_offset..b.data_offset + data_len];
    let first_stop = bytes
        .iter()
        .position(|&c| c == 0 || !is_trim_char(c))
        .unwrap_or(data_len);
    if first_stop > 0 {
        b.data_offset += first_stop;
    }
}

pub fn buffer_trim_right(b: &mut buffer_t) {
    let data_len = content_len(b);
    if data_len == 0 { return; }

    let bytes = &mut b.storage[b.data_offset..b.data_offset + data_len];
    for i in (0..data_len).rev() {
        if bytes[i] == 0 || !is_trim_char(bytes[i]) {
            break;
        }
        bytes[i] = 0u8;
    }
}

pub fn buffer_trim(b: &mut buffer_t) {
    buffer_trim_left(b);
    buffer_trim_right(b);
}

pub fn buffer_fill(b: &mut buffer_t, c: c_int) {
    let byte = c as u8;
    let fill_len = b.len as usize;
    if b.storage.len() < fill_len {
        return;
    }
    // C fills `len` bytes from alloc base, not just the logical string.
    b.storage[..fill_len].fill(byte);
}

pub fn buffer_clear(b: &mut buffer_t) {
    buffer_fill(b, 0);
}

pub fn buffer_print(b: &buffer_t) {
    let len = b.len as usize;
    if b.storage.len() <= len {
        return;
    }
    // C prints from alloc base.
    let bytes = &b.storage[..len];
    print!("\n ");
    for (i, &byte) in bytes.iter().enumerate() {
        print!(" {:02x}", byte);
        if (i + 1) % 8 == 0 { print!("\n "); }
    }
    println!();
}

// ---------------------------------------------------------------------------
// C shims (keep raw pointers, extern "C")
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn buffer_shim_length(ptr: *mut buffer_t) -> size_t {
    if ptr.is_null() {
        return 0;
    }
    // SAFETY: ptr is valid and initialized by the buffer constructor.
    let b = unsafe { &*ptr };
    buffer_length(b)
}

#[no_mangle]
pub extern "C" fn buffer_shim_resize(ptr: *mut buffer_t, n: size_t) -> c_int {
    if ptr.is_null() {
        return -1;
    }
    // SAFETY: ptr is valid and uniquely accessible.
    let b = unsafe { &mut *ptr };
    buffer_resize(b, n)
}

#[no_mangle]
pub extern "C" fn buffer_shim_data_at(ptr: *mut buffer_t, off: size_t) -> *mut c_char {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: ptr is valid.
    let b = unsafe { &*ptr };
    let off = off as usize;
    if (b.data_offset + off) >= b.storage.len() {
        return std::ptr::null_mut();
    }
    // SAFETY: the pointer remains within the owned allocation.
    unsafe { b.storage.as_ptr().add(b.data_offset + off) as *mut c_char }
}
