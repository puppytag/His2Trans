//! Module: src_app_file
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

// === C2R_FILE_STATICS_BEGIN ===
// File-scope `static` variables (internal linkage) from the original C TU.
// These are module-local by design (Scheme B).
/// C: static int32_t g_memoryPageSize
static mut g_memoryPageSize: i32 = 0;

// === C2R_FILE_STATICS_END ===

pub extern "C" fn InitVerify(file: *mut crate::types::FileRead, filePath: *const ::core::ffi::c_char, handle: *mut i32) -> i32 {
    use crate::types::{PATH_MAX, V_OK, V_ERR_FILE_OPEN, V_ERR_MALLOC, V_ERR_FILE_STAT, LOG_CORE, LOG_ERROR, LOG_INFO, SEEK_END};
    use crate::src_app_verify_hal::RegistHalFunc;
    unsafe {
        if handle.is_null() || file.is_null() || filePath.is_null() {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file open error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 32);
            return V_ERR_FILE_OPEN as i32;
        }
        RegistHalFunc();
        let path = libc::malloc(256 + 1) as *mut ::core::ffi::c_char;
        if path.is_null() {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: path malloc error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 38);
            return V_ERR_MALLOC as i32;
        }
        if (libc::strlen(filePath) > 256) || libc::realpath(filePath, path).is_null() {
            if !path.is_null() {
                libc::free(path as *mut ::core::ffi::c_void);
            }
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file path error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 43);
            return V_ERR_FILE_OPEN as i32;
        }
        *handle = libc::open(path, 0, 0);
        if *handle < 0 {
            if !path.is_null() {
                libc::free(path as *mut ::core::ffi::c_void);
            }
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file open error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 49);
            return V_ERR_FILE_OPEN as i32;
        }
        if crate::globals::g_memoryPageSize == 0 {
            crate::globals::g_memoryPageSize = libc::sysconf(30) as i32;
        }
        if crate::globals::g_memoryPageSize <= 0 {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: MAP_FAILED %d\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 56, crate::globals::g_memoryPageSize);
            if !path.is_null() {
                libc::free(path as *mut ::core::ffi::c_void);
            }
            return V_ERR_FILE_STAT as i32;
        }
        (*file).len = libc::lseek(*handle, 0, SEEK_END as i32) as i32;
        (*file).fp = *handle;
        if !path.is_null() {
            libc::free(path as *mut ::core::ffi::c_void);
        }
        V_OK as i32
    }
}

pub extern "C" fn HapMMap(bufCapacity: i32, offset: i32, mmapInfo: *mut crate::types::MmapInfo, file: *const crate::types::FileRead) -> i32 {
    use crate::types::{FILE_IS_CLOSE, READ_OFFSET_OUT_OF_RANGE, MMAP_FAILED, V_OK, SEEK_SET};
    use crate::types::{LOG_CORE, LOG_ERROR};
    use crate::compat::lseek;
    use libc::{mmap, PROT_READ, MAP_SHARED};

    if mmapInfo.is_null() || file.is_null() || bufCapacity <= 0 {
        return MMAP_FAILED;
    }
    unsafe {
        (*mmapInfo).mapAddr = (-1isize) as *mut ::core::ffi::c_char;
    }
    let file_fp = unsafe { (*file).fp };
    if file_fp == -1 {
        return FILE_IS_CLOSE;
    }
    let file_len = unsafe { (*file).len };
    if offset < 0 || offset > file_len - bufCapacity {
        return READ_OFFSET_OUT_OF_RANGE;
    }
    unsafe {
        lseek(file_fp, offset as i64, SEEK_SET as i32);
    }
    let page_size = 4096;
    if page_size == 0 {
        return MMAP_FAILED;
    }
    let mmap_pos = (offset / page_size) * page_size;
    let read_more = offset - mmap_pos;
    let mmap_size = bufCapacity + read_more;
    unsafe {
        (*mmapInfo).mmapPosition = mmap_pos;
        (*mmapInfo).readMoreLen = read_more;
        (*mmapInfo).mmapSize = mmap_size;
        (*mmapInfo).mapAddr = mmap(
            std::ptr::null_mut(),
            mmap_size as usize,
            PROT_READ,
            MAP_SHARED,
            file_fp,
            mmap_pos as i64,
        ) as *mut ::core::ffi::c_char;
        if (*mmapInfo).mapAddr == (-1isize) as *mut ::core::ffi::c_char {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: MAP_FAILED\0".as_ptr() as *const ::core::ffi::c_char,
                b"HapMMap\0".as_ptr() as *const ::core::ffi::c_char,
                88,
            );
            return MMAP_FAILED;
        }
    }
    V_OK as i32
}

pub extern "C" fn HapMUnMap(mapAddr: *mut ::core::ffi::c_char, mmapSize: i32) {
    if mapAddr.is_null() || mmapSize <= 0 {
        return;
    }
    unsafe {
        libc::munmap(mapAddr as *mut ::core::ffi::c_void, mmapSize as usize);
    }
}
