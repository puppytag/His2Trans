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
    if handle.is_null() || file.is_null() || filePath.is_null() {
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    crate::src_app_verify_hal::RegistHalFunc();
    
    let path = unsafe { libc::malloc((crate::types::PATH_MAX + 1) as usize) as *mut ::core::ffi::c_char };
    if path.is_null() {
        return crate::types::V_ERR_MALLOC as i32;
    }
    
    let path_len = unsafe { libc::strlen(filePath) };
    if path_len > crate::types::PATH_MAX as usize || unsafe { libc::realpath(filePath, path) }.is_null() {
        unsafe {
            libc::free(path as *mut ::core::ffi::c_void);
        }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    unsafe {
        *handle = libc::open(path, libc::O_RDONLY, 0);
    }
    
    if unsafe { *handle } < 0 {
        unsafe {
            libc::free(path as *mut ::core::ffi::c_void);
        }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    unsafe {
        if crate::globals::g_memoryPageSize == 0 {
            crate::globals::g_memoryPageSize = libc::sysconf(libc::_SC_PAGESIZE) as i32;
        }
    }
    
    if unsafe { crate::globals::g_memoryPageSize } <= 0 {
        unsafe {
            libc::free(path as *mut ::core::ffi::c_void);
        }
        return crate::types::V_ERR_FILE_STAT as i32;
    }
    
    unsafe {
        (*file).len = libc::lseek(*handle, 0, crate::types::SEEK_END as i32) as i32;
        (*file).fp = *handle;
        libc::free(path as *mut ::core::ffi::c_void);
    }
    
    crate::types::V_OK as i32
}

pub extern "C" fn HapMMap(bufCapacity: i32, offset: i32, mmapInfo: *mut crate::types::MmapInfo, file: *const crate::types::FileRead) -> i32 {
    use crate::types::*;
    use crate::globals::*;
    
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
        
        libc::lseek((*file).fp, offset as libc::off_t, libc::SEEK_SET as i32);
        
        if g_memoryPageSize == 0 {
            return MMAP_FAILED;
        }
        
        (*mmapInfo).mmapPosition = (offset / g_memoryPageSize) * g_memoryPageSize;
        (*mmapInfo).readMoreLen = offset - (*mmapInfo).mmapPosition;
        (*mmapInfo).mmapSize = bufCapacity + (*mmapInfo).readMoreLen;
        
        (*mmapInfo).mapAddr = libc::mmap(
            std::ptr::null_mut(),
            (*mmapInfo).mmapSize as usize,
            libc::PROT_READ,
            libc::MAP_SHARED,
            (*file).fp,
            (*mmapInfo).mmapPosition as libc::off_t
        ) as *mut ::core::ffi::c_char;
        
        if (*mmapInfo).mapAddr == (-1isize) as *mut ::core::ffi::c_char {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: MAP_FAILED\0".as_ptr() as *const ::core::ffi::c_char,
                b"HapMMap\0".as_ptr() as *const ::core::ffi::c_char,
                88i32
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
