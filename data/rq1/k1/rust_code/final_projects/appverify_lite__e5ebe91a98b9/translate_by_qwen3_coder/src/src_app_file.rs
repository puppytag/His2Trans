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
    use crate::types::{PATH_MAX, V_ERR_FILE_OPEN, V_ERR_MALLOC, V_ERR_FILE_STAT, V_OK, LOG_CORE, LOG_ERROR, LOG_INFO, SEEK_END};
    use crate::src_app_verify_hal::RegistHalFunc;
    if handle.is_null() || file.is_null() || filePath.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file open error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 32) };
        return V_ERR_FILE_OPEN as i32;
    }
    RegistHalFunc();
    let path = unsafe { libc::malloc((PATH_MAX as usize) + 1) as *mut ::core::ffi::c_char };
    if path.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: path malloc error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 38) };
        return V_ERR_MALLOC as i32;
    }
    if unsafe { libc::strlen(filePath) as u64 } > PATH_MAX as u64 || unsafe { crate::compat::realpath(filePath, path) }.is_null() {
        unsafe { libc::free(path as *mut _) };
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file path error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 43) };
        return V_ERR_FILE_OPEN as i32;
    }
    unsafe { *handle = crate::compat::open(path, 0, 0) };
    if unsafe { *handle } < 0 {
        unsafe { libc::free(path as *mut _) };
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: file open error\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 49) };
        return V_ERR_FILE_OPEN as i32;
    }
    unsafe {
        let page_size = crate::compat::sysconf(30);
        if page_size <= 0 {
            let _ = crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: MAP_FAILED %d\0".as_ptr() as *const _, b"InitVerify\0".as_ptr() as *const _, 56, page_size);
            libc::free(path as *mut _);
            return V_ERR_FILE_STAT as i32;
        }
        (*file).len = crate::compat::lseek(*handle, 0, SEEK_END as i32) as i32;
        (*file).fp = *handle;
        libc::free(path as *mut _);
    }
    V_OK as i32
}

pub extern "C" fn HapMMap(bufCapacity: i32, offset: i32, mmapInfo: *mut crate::types::MmapInfo, file: *const crate::types::FileRead) -> i32 {
    use crate::types::{FILE_IS_CLOSE, READ_OFFSET_OUT_OF_RANGE, MMAP_FAILED, V_OK, LOG_CORE, LOG_ERROR};
    if mmapInfo.is_null() || file.is_null() || bufCapacity <= 0 {
        return MMAP_FAILED;
    }
    unsafe {
        (*mmapInfo).mapAddr = (-1isize) as *mut ::core::ffi::c_char;
        if (*file).fp == -1 {
            return FILE_IS_CLOSE;
        }
        if offset < 0 || offset > (*file).len - bufCapacity {
            return READ_OFFSET_OUT_OF_RANGE;
        }
        let _ = crate::compat::lseek((*file).fp, offset as crate::types::off_t, 0);
        let g_memoryPageSize_var = crate::compat::sysconf(4) as i32;
        if g_memoryPageSize == 0 {
            return MMAP_FAILED;
        }
        (*mmapInfo).mmapPosition = (offset / g_memoryPageSize) * g_memoryPageSize;
        (*mmapInfo).readMoreLen = offset - (*mmapInfo).mmapPosition;
        (*mmapInfo).mmapSize = bufCapacity + (*mmapInfo).readMoreLen;
        (*mmapInfo).mapAddr = crate::compat::mmap(
            std::ptr::null_mut(),
            (*mmapInfo).mmapSize as crate::types::size_t,
            1,
            0x01,
            (*file).fp,
            (*mmapInfo).mmapPosition as crate::types::off_t,
        ) as *mut ::core::ffi::c_char;
        if (*mmapInfo).mapAddr == (-1isize) as *mut ::core::ffi::c_char {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const ::core::ffi::c_char,
                "[%s:%d]: MAP_FAILED\0".as_ptr() as *const ::core::ffi::c_char,
                "HapMMap\0".as_ptr() as *const ::core::ffi::c_char,
                88,
            );
            return MMAP_FAILED;
        }
        V_OK as i32
    }
}

pub extern "C" fn HapMUnMap(mapAddr: *mut ::core::ffi::c_char, mmapSize: i32) {
    if mapAddr.is_null() || mmapSize <= 0 {
        return;
    }
    unsafe {
        libc::munmap(mapAddr as *mut ::core::ffi::c_void, mmapSize as usize);
    }
}
