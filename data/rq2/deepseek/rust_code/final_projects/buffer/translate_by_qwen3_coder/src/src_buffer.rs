//! Module: src_buffer
//!
//! Auto-generated skeleton - function bodies are unimplemented.

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::types::*;
use crate::globals::*;
use crate::compat::*;

pub extern "C" fn buffer_new() -> *mut crate::types::buffer_t {
    crate::src_buffer::buffer_new_with_size(64)
}

pub extern "C" fn buffer_new_with_size(n: crate::types::size_t) -> *mut crate::types::buffer_t {
    unsafe {
        let self_ = libc::malloc(std::mem::size_of::<crate::types::buffer_t>()) as *mut crate::types::buffer_t;
        if self_.is_null() {
            return std::ptr::null_mut();
        }
        let data = libc::calloc(n.wrapping_add(1) as usize, 1) as *mut ::core::ffi::c_void;
        if data.is_null() {
            libc::free(self_ as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        std::ptr::write_bytes(self_ as *mut u8, 0, std::mem::size_of::<crate::types::buffer_t>());
        self_
    }
}

pub extern "C" fn buffer_new_with_string(str_: *mut ::core::ffi::c_char) -> *mut crate::types::buffer_t {
    unsafe {
        let len = libc::strlen(str_) as crate::types::size_t;
        crate::src_buffer::buffer_new_with_string_length(str_, len)
    }
}

pub extern "C" fn buffer_new_with_string_length(str_: *mut ::core::ffi::c_char, len: crate::types::size_t) -> *mut crate::types::buffer_t {
    let self_ = unsafe { libc::malloc(std::mem::size_of::<crate::types::buffer_t>()) } as *mut crate::types::buffer_t;
    if self_.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        *self_.cast::<*mut ::core::ffi::c_char>().offset(0) = str_;
        *self_.cast::<*mut ::core::ffi::c_char>().offset(1) = str_;
        *self_.cast::<crate::types::size_t>().offset(2) = len;
    }
    self_
}

pub extern "C" fn buffer_new_with_copy(str_: *mut ::core::ffi::c_char) -> *mut crate::types::buffer_t {
    unsafe {
        let len = libc::strlen(str_) as crate::types::size_t;
        let self_ = crate::src_buffer::buffer_new_with_size(len);
        if self_.is_null() {
            return std::ptr::null_mut();
        }
        std::ptr::copy_nonoverlapping(str_, (*self_).alloc as *mut ::core::ffi::c_char, len as usize);
        (*self_).data = (*self_).alloc;
        self_
    }
}

pub extern "C" fn buffer_compact(self_: *mut crate::types::buffer_t) -> crate::types::ssize_t {
    let len = crate::src_buffer::buffer_length(self_);
    let len_ptr = unsafe {
        crate::compat::c2r_field_ptr_buffer_t__len(self_ as *mut ::core::ffi::c_void)
    };
    let self_len = unsafe { *(len_ptr as *const crate::types::size_t) };
    let rem = (self_len as isize).wrapping_sub(len as isize);
    let buf = unsafe { libc::calloc(len.wrapping_add(1) as usize, 1) as *mut ::core::ffi::c_char };
    if buf.is_null() {
        return -1;
    }
    let data_ptr = unsafe {
        crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void)
    };
    let self_data = unsafe { *(data_ptr as *const *mut ::core::ffi::c_char) };
    unsafe {
        std::ptr::copy_nonoverlapping(self_data, buf, len as usize);
    }
    let alloc_ptr = unsafe {
        crate::compat::c2r_field_ptr_buffer_t__alloc(self_ as *mut ::core::ffi::c_void)
    };
    let self_alloc = unsafe { *(alloc_ptr as *const *mut ::core::ffi::c_void) };
    unsafe {
        libc::free(self_alloc);
    }
    unsafe {
        *(len_ptr as *mut crate::types::size_t) = len;
        *(data_ptr as *mut *mut ::core::ffi::c_char) = buf;
        *(alloc_ptr as *mut *mut ::core::ffi::c_void) = buf as *mut ::core::ffi::c_void;
    }
    rem as crate::types::ssize_t
}

pub extern "C" fn buffer_free(self_: *mut crate::types::buffer_t) {
    if !self_.is_null() {
        let alloc_ptr = unsafe {
            crate::compat::c2r_field_ptr_buffer_t__alloc(self_ as *mut ::core::ffi::c_void)
        };
        if !alloc_ptr.is_null() {
            unsafe {
                libc::free(alloc_ptr as *mut ::core::ffi::c_void);
            }
        }
        unsafe {
            libc::free(self_ as *mut ::core::ffi::c_void);
        }
    }
}

pub extern "C" fn buffer_size(self_: *mut crate::types::buffer_t) -> crate::types::size_t {
    unsafe {
        let len_ptr = crate::compat::c2r_field_ptr_buffer_t__len(self_ as *mut ::core::ffi::c_void) as *mut crate::types::size_t;
        *len_ptr
    }
}

pub extern "C" fn buffer_length(self_: *mut crate::types::buffer_t) -> crate::types::size_t {
    unsafe {
        let data_ptr = crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
        if data_ptr.is_null() {
            return 0;
        }
        libc::strlen(data_ptr) as crate::types::size_t
    }
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_buffer_10
// c_function: buffer_resize
// rust_file: src_buffer.rs
// rust_signature: pub extern "C" fn buffer_resize(self_: *mut crate::types::buffer_t, n: crate::types::size_t) -> ::core::ffi::c_int
// c_first_line: int
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/buffer/workspace/repair_history/buffer/translate_by_qwen3_coder/_manual_fix/src_buffer_10/translated_rust.rs
// last_error_truncated:
//   error[E0308]: mismatched types
//       --> src/src_buffer.rs:96:25
//        |
//        |            ------------ ^^^^ expected `usize`, found `u64`
//        |            |
//        |            arguments to this method are incorrect
//        |
//       --> /data/home/wangshb/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs:2375:22
// =================================
pub extern "C" fn buffer_resize(self_: *mut crate::types::buffer_t, n: crate::types::size_t) -> ::core::ffi::c_int {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_buffer_10
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/buffer/workspace/repair_history/buffer/translate_by_qwen3_coder/_manual_fix/src_buffer_10/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn buffer_resize(self_: *mut crate::types::buffer_t, n: crate::types::size_t) -> ::core::ffi::c_int {
    let mut n = n;
    let mask = 1024u64 - 1;
    n = (n.wrapping_add(mask) & !mask) as crate::types::size_t;
    unsafe {
        let len_ptr = crate::compat::c2r_field_ptr_buffer_t__len(self_ as *mut ::core::ffi::c_void) as *mut crate::types::size_t;
        *len_ptr = n;
        let alloc_ptr = crate::compat::c2r_field_ptr_buffer_t__alloc(self_ as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_char;
        let new_alloc = libc::realloc(*alloc_ptr as *mut ::core::ffi::c_void, (n as usize).wrapping_add(1));
        *alloc_ptr = new_alloc as *mut ::core::ffi::c_char;
        let data_ptr = crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_char;
        *data_ptr = new_alloc as *mut ::core::ffi::c_char;
        if new_alloc.is_null() {
            return -1;
        }
        *(new_alloc as *mut ::core::ffi::c_char).offset(n as isize) = 0;
    }
    0
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_buffer_10
 * === C2R_LLM_FAILED_OUTPUT_END === */


// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_buffer_11
// c_function: buffer_appendf
// rust_file: src_buffer.rs
// rust_signature: pub extern "C" fn buffer_appendf(self_: *mut crate::types::buffer_t, format: *const std::ffi::c_char) -> i32
// c_first_line: int buffer_appendf(buffer_t *self, const char *format, ...) {
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/buffer/workspace/repair_history/buffer/translate_by_qwen3_coder/_manual_fix/src_buffer_11/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find function `vsnprintf` in crate `libc`
//      --> src/src_buffer.rs:201:31
//       |
//       |                               ^^^^^^^^^
//       |
//       |
//       |     -------------------------------------------------------------------------------- similarly named function `snprintf` defined here
//       |
// =================================
pub extern "C" fn buffer_appendf(self_: *mut crate::types::buffer_t, format: *const std::ffi::c_char) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_buffer_11
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/Our/deepseek-coder/intermediate/buffer/workspace/repair_history/buffer/translate_by_qwen3_coder/_manual_fix/src_buffer_11/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn buffer_appendf(self_: *mut crate::types::buffer_t, format: *const std::ffi::c_char) -> i32 {
    let mut ap: std::ffi::VaList;
    let mut tmpa: std::ffi::VaList;
    let mut dst: *mut std::ffi::c_char = std::ptr::null_mut();
    let mut length: i32 = 0;
    let mut required: i32 = 0;
    let mut bytes: i32 = 0;

    unsafe {
        ap = std::ffi::VaList::new(format);
    }

    length = crate::src_buffer::buffer_length(self_) as i32;

    unsafe {
        tmpa = ap.clone();
    }
    required = unsafe { libc::vsnprintf(std::ptr::null_mut(), 0, format, tmpa.as_va_list()) };
    if crate::src_buffer::buffer_resize(self_, (length as u64).wrapping_add(required as u64)) == -1 {
        return -1;
    }

    unsafe {
        dst = (*self_).data.offset(length as isize);
    }
    bytes = unsafe { libc::vsnprintf(dst, (required + 1) as usize, format, ap.as_va_list()) };

    if bytes < 0 {
        -1
    } else {
        0
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_buffer_11
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn buffer_append(self_: *mut crate::types::buffer_t, str_: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        let len = libc::strlen(str_) as crate::types::size_t;
        crate::src_buffer::buffer_append_n(self_, str_, len)
    }
}

pub extern "C" fn buffer_append_n(self_: *mut crate::types::buffer_t, str_: *const ::core::ffi::c_char, len: crate::types::size_t) -> ::core::ffi::c_int {
    unsafe {
        let data_ptr = crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
        let prev = libc::strlen(data_ptr) as usize;
        let needed = len as usize + prev;
        let len_ptr = crate::compat::c2r_field_ptr_buffer_t__len(self_ as *mut ::core::ffi::c_void) as *mut crate::types::size_t;
        let current_len = *len_ptr as usize;
        if current_len > needed {
            libc::strncat(data_ptr, str_, len as usize);
            return 0;
        }
        let ret = crate::src_buffer::buffer_resize(self_, needed as crate::types::size_t);
        if ret == -1 {
            return -1;
        }
        let new_data_ptr = crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
        libc::strncat(new_data_ptr, str_, len as usize);
        0
    }
}

pub extern "C" fn buffer_prepend(self_: *mut crate::types::buffer_t, str_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    unsafe {
        let len = libc::strlen(str_) as crate::types::size_t;
        let self_data = (*self_).data;
        let prev = libc::strlen(self_data) as crate::types::size_t;
        let needed = len + prev;
        let self_len = (*self_).alloc;
        if !self_len.is_null() {
            let alloc_len = libc::strlen(self_len) as crate::types::size_t;
            if alloc_len > needed {
                std::ptr::copy(self_data, self_data.offset(len as isize), prev + 1);
                std::ptr::copy_nonoverlapping(str_, self_data, len);
                return 0;
            }
        }
        let ret = crate::src_buffer::buffer_resize(self_, needed);
        if ret == -1 {
            return -1;
        }
        let self_data = (*self_).data;
        std::ptr::copy(self_data, self_data.offset(len as isize), prev + 1);
        std::ptr::copy_nonoverlapping(str_, self_data, len);
        0
    }
}

pub extern "C" fn buffer_slice(self_: *mut crate::types::buffer_t, from: crate::types::size_t, to: crate::types::ssize_t) -> *mut crate::types::buffer_t {
    let buf_data_ptr = unsafe {
        crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void)
    };
    let buf_data = buf_data_ptr as *mut ::core::ffi::c_char;
    let len = unsafe { libc::strlen(buf_data) } as usize;
    let mut to_i = to as isize;
    if to_i < from as isize {
        return std::ptr::null_mut();
    }
    if to_i < 0 {
        to_i = (len as isize) - !to_i;
    }
    if to_i > len as isize {
        to_i = len as isize;
    }
    let n = (to_i - from as isize) as usize;
    let self_ptr = crate::src_buffer::buffer_new_with_size(n as crate::types::size_t);
    if self_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let self_data_ptr = unsafe {
        crate::compat::c2r_field_ptr_buffer_t__data(self_ptr as *mut ::core::ffi::c_void)
    };
    let self_data = self_data_ptr as *mut ::core::ffi::c_char;
    unsafe {
        std::ptr::copy_nonoverlapping(buf_data.offset(from as isize), self_data, n);
        let terminator = self_data.offset(n as isize);
        *terminator = 0;
    }
    self_ptr
}

pub extern "C" fn buffer_equals(self_: *mut crate::types::buffer_t, other: *mut crate::types::buffer_t) -> ::core::ffi::c_int {
    if self_.is_null() || other.is_null() {
        return 0;
    }
    unsafe {
        let self_data = self_ as *mut ::core::ffi::c_char;
        let other_data = other as *mut ::core::ffi::c_char;
        (libc::strcmp(self_data, other_data) == 0) as ::core::ffi::c_int
    }
}

pub extern "C" fn buffer_indexof(self_: *mut crate::types::buffer_t, str_: *mut ::core::ffi::c_char) -> crate::types::ssize_t {
    let data_ptr = unsafe {
        crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char
    };
    let sub = unsafe { libc::strstr(data_ptr, str_) };
    if sub.is_null() {
        return -1;
    }
    unsafe { (sub as isize - data_ptr as isize) as crate::types::ssize_t }
}

pub extern "C" fn buffer_trim_left(self_: *mut crate::types::buffer_t) {
    unsafe {
        let mut data_ptr = crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
        loop {
            let c = *data_ptr;
            if c == 0 {
                break;
            }
            if libc::isspace(c as ::core::ffi::c_int) == 0 {
                break;
            }
            data_ptr = data_ptr.add(1);
        }
        *(crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_char) = data_ptr;
    }
}

pub extern "C" fn buffer_trim_right(self_: *mut crate::types::buffer_t) {
    if self_.is_null() {
        return;
    }
    let len = crate::src_buffer::buffer_length(self_);
    if len == 0 {
        return;
    }
    let mut i = len - 1;
    let data_ptr = unsafe {
        crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void)
            as *mut ::core::ffi::c_char
    };
    loop {
        let c = unsafe { *data_ptr.offset(i as isize) };
        if c == 0 {
            break;
        }
        if unsafe { libc::isspace(c as i32) } == 0 {
            break;
        }
        unsafe {
            *data_ptr.offset(i as isize) = 0;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
}

pub extern "C" fn buffer_trim(self_: *mut crate::types::buffer_t) {
    crate::src_buffer::buffer_trim_left(self_);
    crate::src_buffer::buffer_trim_right(self_);
}

pub extern "C" fn buffer_fill(self_: *mut crate::types::buffer_t, c: ::core::ffi::c_int) {
    if self_.is_null() {
        return;
    }
    unsafe {
        let data_ptr = crate::compat::c2r_field_ptr_buffer_t__data(self_ as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
        let len_ptr = crate::compat::c2r_field_ptr_buffer_t__len(self_ as *mut ::core::ffi::c_void) as *mut crate::types::size_t;
        let len = *len_ptr as usize;
        std::ptr::write_bytes(data_ptr, c as u8, len);
    }
}

pub extern "C" fn buffer_clear(self_: *mut crate::types::buffer_t) {
    crate::src_buffer::buffer_fill(self_, 0);
}

pub extern "C" fn buffer_print(self_: *mut crate::types::buffer_t) {
    if self_.is_null() {
        return;
    }
    let len_ptr = unsafe {
        crate::compat::c2r_field_ptr_buffer_t__len(self_ as *mut ::core::ffi::c_void)
    };
    let len = unsafe { *(len_ptr as *const crate::types::size_t) };
    let alloc_ptr = unsafe {
        crate::compat::c2r_field_ptr_buffer_t__alloc(self_ as *mut ::core::ffi::c_void)
    };
    let alloc = unsafe { *(alloc_ptr as *mut *mut ::core::ffi::c_char) };
    unsafe {
        crate::compat::printf("\n \0".as_ptr() as *const i8);
    }
    for i in 0..(len as i32) {
        unsafe {
            crate::compat::printf(
                " %02x\0".as_ptr() as *const i8,
                *alloc.offset(i as isize) as i32 & 0xff,
            );
        }
        if (i + 1) % 8 == 0 {
            unsafe {
                crate::compat::printf("\n \0".as_ptr() as *const i8);
            }
        }
    }
    unsafe {
        crate::compat::printf("\n\0".as_ptr() as *const i8);
    }
}
