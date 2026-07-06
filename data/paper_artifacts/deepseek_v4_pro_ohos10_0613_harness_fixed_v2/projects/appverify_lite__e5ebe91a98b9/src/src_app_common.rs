//! Module: src_app_common
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

pub extern "C" fn HapGetInt64(buf: *const ::core::ffi::c_uchar, len: i32) -> ::core::ffi::c_longlong {
    if buf.is_null() {
        return 0;
    }
    let mut value: u64 = 0;
    if len != (std::mem::size_of::<::core::ffi::c_longlong>() as i32) {
        return 0;
    }
    let mut remaining = len;
    while remaining > 0 {
        remaining -= 1;
        value = (value << 8) | (unsafe { *buf.offset(remaining as isize) } as u64);
    }
    value as ::core::ffi::c_longlong
}

pub extern "C" fn HapGetInt(buf: *const ::core::ffi::c_uchar, len: i32) -> i32 {
    let value: u32 = HapGetUnsignedInt(buf, len);
    value as i32
}

pub extern "C" fn HapGetUnsignedInt(buf: *const ::core::ffi::c_uchar, len: i32) -> u32 {
    if buf.is_null() {
        return 0;
    }
    if len != core::mem::size_of::<::core::ffi::c_int>() as i32 {
        return 0;
    }
    let mut len_mut = len;
    let mut value: u32 = 0;
    while len_mut > 0 {
        len_mut -= 1;
        unsafe {
            value = (value << 8) | (*buf.add(len_mut as usize) as u32);
        }
    }
    value
}

pub extern "C" fn HapGetShort(buf: *const ::core::ffi::c_uchar, len: i32) -> ::core::ffi::c_short {
    if buf.is_null() {
        return 0;
    }
    let size = std::mem::size_of::<::core::ffi::c_short>() as i32;
    if len != size {
        return 0;
    }
    let mut value: u16 = 0;
    let mut remaining = len;
    unsafe {
        while remaining > 0 {
            remaining -= 1;
            let byte = *buf.offset(remaining as isize);
            value = (value << 8) | (byte as u16);
        }
    }
    value as ::core::ffi::c_short
}

pub extern "C" fn HapPutInt32(buf: *mut ::core::ffi::c_uchar, len: i32, value: i32) {
    let size_of_int = core::mem::size_of::<i32>();
    if buf.is_null() || len < (size_of_int as i32) {
        return;
    }
    let mut var = value as u32;
    for i in 0..size_of_int {
        unsafe {
            *buf.add(i) = var as u8;
        }
        var >>= 8;
    }
}
