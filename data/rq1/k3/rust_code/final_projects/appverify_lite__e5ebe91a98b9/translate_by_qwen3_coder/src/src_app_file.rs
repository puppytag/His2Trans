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
    use crate::types::{V_ERR_FILE_OPEN, V_ERR_MALLOC, V_ERR_FILE_STAT, V_OK, PATH_MAX, SEEK_END};
    use crate::src_app_verify_hal::RegistHalFunc;
    if handle.is_null() || file.is_null() || filePath.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: file open error\0".as_ptr() as *const _,
            b"InitVerify\0".as_ptr() as *const _,
            32,
        ) };
        return V_ERR_FILE_OPEN as i32;
    }
    unsafe {
        RegistHalFunc();
    }
    let path = unsafe { libc::malloc((PATH_MAX as usize + 1) as usize) } as *mut ::core::ffi::c_char;
    if path.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: path malloc error\0".as_ptr() as *const _,
            b"InitVerify\0".as_ptr() as *const _,
            38,
        ) };
        return V_ERR_MALLOC as i32;
    }
    let file_path_len = unsafe { libc::strlen(filePath) };
    if file_path_len > PATH_MAX as usize || unsafe { crate::compat::realpath(filePath, path) }.is_null() {
        if !path.is_null() {
            unsafe { libc::free(path as *mut ::core::ffi::c_void) };
        }
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: file path error\0".as_ptr() as *const _,
            b"InitVerify\0".as_ptr() as *const _,
            43,
        ) };
        return V_ERR_FILE_OPEN as i32;
    }
    unsafe { *handle = crate::compat::open(path, 0, 0) };
    if unsafe { *handle } < 0 {
        if !path.is_null() {
            unsafe { libc::free(path as *mut ::core::ffi::c_void) };
        }
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: file open error\0".as_ptr() as *const _,
            b"InitVerify\0".as_ptr() as *const _,
            49,
        ) };
        return V_ERR_FILE_OPEN as i32;
    }
    unsafe {
        static mut G_MEMORY_PAGE_SIZE: i64 = 0;
        if G_MEMORY_PAGE_SIZE == 0 {
            G_MEMORY_PAGE_SIZE = crate::compat::sysconf(30);
        }
        if G_MEMORY_PAGE_SIZE <= 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: MAP_FAILED %d\0".as_ptr() as *const _,
                b"InitVerify\0".as_ptr() as *const _,
                56,
                G_MEMORY_PAGE_SIZE as i32,
            );
            if !path.is_null() {
                libc::free(path as *mut ::core::ffi::c_void);
            }
            return V_ERR_FILE_STAT as i32;
        }
        (*file).len = crate::compat::lseek(*handle, 0, SEEK_END as i32) as i32;
        (*file).fp = *handle;
        if !path.is_null() {
            libc::free(path as *mut ::core::ffi::c_void);
        }
    }
    V_OK as i32
}

pub extern "C" fn HapMMap(bufCapacity: i32, offset: i32, mmapInfo: *mut crate::types::MmapInfo, file: *const crate::types::FileRead) -> i32 {
    use crate::types::{FILE_IS_CLOSE, MMAP_FAILED, READ_OFFSET_OUT_OF_RANGE, V_OK};
    use crate::compat::{lseek, mmap};
    use libc::{PROT_READ, MAP_SHARED, MAP_FAILED};
    use core::ffi::c_void;

    if mmapInfo.is_null() || file.is_null() || bufCapacity <= 0 {
        return MMAP_FAILED;
    }
    unsafe {
        (*mmapInfo).mapAddr = MAP_FAILED as *mut ::core::ffi::c_char;
    }
    let fp = unsafe { (*file).fp };
    if fp == -1 {
        return FILE_IS_CLOSE;
    }
    let len = unsafe { (*file).len };
    if offset < 0 || offset > len - bufCapacity {
        return READ_OFFSET_OUT_OF_RANGE;
    }
    unsafe {
        lseek(fp, offset as i64, 0);
    }
    let page_size = unsafe { crate::compat::g_memoryPageSize };
    if page_size == 0 {
        return MMAP_FAILED;
    }
    let mmap_position = (offset / page_size) * page_size;
    let read_more_len = offset - mmap_position;
    let mmap_size = bufCapacity + read_more_len;
    let map_addr = unsafe {
        mmap(
            core::ptr::null_mut(),
            mmap_size as u32,
            PROT_READ as i32,
            MAP_SHARED as i32,
            fp,
            mmap_position as i64,
        ) as *mut ::core::ffi::c_char
    };
    unsafe {
        (*mmapInfo).mmapPosition = mmap_position;
        (*mmapInfo).readMoreLen = read_more_len;
        (*mmapInfo).mmapSize = mmap_size;
        (*mmapInfo).mapAddr = map_addr;
    }
    if map_addr as *mut c_void == MAP_FAILED {
        return MMAP_FAILED;
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
