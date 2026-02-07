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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_app_file_1
// c_function: InitVerify
// rust_file: src_app_file.rs
// rust_signature: pub extern "C" fn InitVerify(file: *mut crate::types::FileRead, filePath: *const ::core::ffi::c_char, handle: *mut i32) -> i32
// c_first_line: int32_t InitVerify(FileRead *file, const char *filePath, int32_t *handle)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_file_1/translated_rust.rs
// last_error_truncated:
//   error[E0599]: no method named `as_ptr` found for type `i32` in the current scope
//      --> src/src_app_verify.rs:216:55
//       |
//       |                                                       ^^^^^^ method not found in `i32`
//   error: could not compile `appverify_lite__e5ebe91a98b9` (bin "appverify_lite__e5ebe91a98b9") due to 1 previous error; 4 warnings emitted
// =================================
pub extern "C" fn InitVerify(file: *mut crate::types::FileRead, filePath: *const ::core::ffi::c_char, handle: *mut i32) -> i32 {
    unimplemented!()
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_app_file_1
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_nosed/intermediate/appverify_lite__e5ebe91a98b9/workspace/repair_history/appverify_lite__e5ebe91a98b9/translate_by_qwen3_coder/_manual_fix/src_app_file_1/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn InitVerify(file: *mut crate::types::FileRead, filePath: *const ::core::ffi::c_char, handle: *mut i32) -> i32 {
    if handle.is_null() || file.is_null() || filePath.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: file open error\0".as_ptr() as *const ::core::ffi::c_char,
                b"InitVerify\0".as_ptr() as *const ::core::ffi::c_char,
                32i32,
            );
        }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    crate::src_app_verify_hal::RegistHalFunc();
    
    let path: *mut ::core::ffi::c_char = unsafe { libc::malloc((crate::types::PATH_MAX + 1) as usize) } as *mut ::core::ffi::c_char;
    if path.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: path malloc error\0".as_ptr() as *const ::core::ffi::c_char,
                b"InitVerify\0".as_ptr() as *const ::core::ffi::c_char,
                38i32,
            );
        }
        return crate::types::V_ERR_MALLOC as i32;
    }
    
    let path_len = unsafe { libc::strlen(filePath) };
    if path_len > crate::types::PATH_MAX as usize || unsafe { libc::realpath(filePath, path) }.is_null() {
        unsafe { libc::free(path as *mut ::core::ffi::c_void) };
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: file path error\0".as_ptr() as *const ::core::ffi::c_char,
                b"InitVerify\0".as_ptr() as *const ::core::ffi::c_char,
                43i32,
            );
        }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    unsafe { *handle = libc::open(path, 0, 0) };
    if unsafe { *handle } < 0 {
        unsafe { libc::free(path as *mut ::core::ffi::c_void) };
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: file open error\0".as_ptr() as *const ::core::ffi::c_char,
                b"InitVerify\0".as_ptr() as *const ::core::ffi::c_char,
                49i32,
            );
        }
        return crate::types::V_ERR_FILE_OPEN as i32;
    }
    
    unsafe {
        if crate::globals::g_memoryPageSize == 0 {
            crate::globals::g_memoryPageSize = libc::sysconf(30) as i32;
        }
    }
    
    if unsafe { crate::globals::g_memoryPageSize } <= 0 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: MAP_FAILED %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"InitVerify\0".as_ptr() as *const ::core::ffi::c_char,
                56i32,
                crate::globals::g_memoryPageSize,
            );
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
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_app_file_1
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn HapMMap(bufCapacity: i32, offset: i32, mmapInfo: *mut crate::types::MmapInfo, file: *const crate::types::FileRead) -> i32 {
    use crate::types::*;
    
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
        
        libc::lseek((*file).fp, offset as libc::off_t, SEEK_SET as i32);
        
        if crate::globals::g_memoryPageSize == 0 {
            return MMAP_FAILED;
        }
        
        let page_size = crate::globals::g_memoryPageSize;
        (*mmapInfo).mmapPosition = (offset / page_size) * page_size;
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
            let _ = crate::compat::HiLogPrint(
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
