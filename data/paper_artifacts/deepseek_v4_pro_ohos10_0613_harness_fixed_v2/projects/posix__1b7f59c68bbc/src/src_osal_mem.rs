//! Module: src_osal_mem
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

pub extern "C" fn OsalMemAlloc(size: crate::types::size_t) -> *mut ::core::ffi::c_void {
    if size == 0 {
        return ::core::ptr::null_mut();
    }
    if size > (0xffffffffu32 as crate::types::size_t) {
        return ::core::ptr::null_mut();
    }
    unsafe { ::libc::malloc(size as usize) as *mut ::core::ffi::c_void }
}

pub extern "C" fn OsalMemCalloc(size: crate::types::size_t) -> *mut ::core::ffi::c_void {
    if size == 0 {
        return ::core::ptr::null_mut();
    }
    if size > (0xFFFF_FFFFu32 as crate::types::size_t) {
        return ::core::ptr::null_mut();
    }
    let buf = crate::src_osal_mem::OsalMemAlloc(size);
    if !buf.is_null() {
        unsafe {
            ::core::ptr::write_bytes(buf as *mut u8, 0u8, size as usize);
        }
    }
    buf
}

pub extern "C" fn OsalMemAllocAlign(alignment: crate::types::size_t, size: crate::types::size_t) -> *mut ::core::ffi::c_void {
    if size == 0 {
        return ::core::ptr::null_mut();
    }
    let mut buf: *mut ::core::ffi::c_void = ::core::ptr::null_mut();
    let ret = unsafe { crate::compat::posix_memalign(&mut buf, alignment, size) };
    if ret != 0 {
        return ::core::ptr::null_mut();
    }
    buf
}

pub extern "C" fn OsalMemFree(mem: *mut ::core::ffi::c_void) {
    if !mem.is_null() {
        unsafe { free(mem); }
    }
}
