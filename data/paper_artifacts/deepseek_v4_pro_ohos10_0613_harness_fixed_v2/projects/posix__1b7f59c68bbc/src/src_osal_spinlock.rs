//! Module: src_osal_spinlock
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

pub extern "C" fn OsalSpinInit(spinlock: *mut OsalSpinlock) -> i32 {
    if spinlock.is_null() {
        let tag = b"osal_spinlock\0".as_ptr() as *const i8;
        let fmt = b"%s invalid param\0".as_ptr() as *const i8;
        let func = b"OsalSpinInit\0".as_ptr() as *const i8;
        let log_type = LOG_CORE as ::core::ffi::c_uint;
        let log_level = LOG_ERROR as ::core::ffi::c_uint;
        unsafe {
            HiLogPrint(log_type, log_level, 0xD002510u32, tag, fmt, func);
        }
        return HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        (*spinlock).realSpinlock = std::ptr::null_mut();
    }
    let size = std::mem::size_of::<libc::pthread_spinlock_t>() as crate::types::size_t;
    let spin_tmp = crate::src_osal_mem::OsalMemCalloc(size) as *mut libc::pthread_spinlock_t;
    if spin_tmp.is_null() {
        let tag = b"osal_spinlock\0".as_ptr() as *const i8;
        let fmt = b"malloc fail\0".as_ptr() as *const i8;
        let log_type = LOG_CORE as ::core::ffi::c_uint;
        let log_level = LOG_ERROR as ::core::ffi::c_uint;
        unsafe {
            HiLogPrint(log_type, log_level, 0xD002510u32, tag, fmt);
        }
        return HDF_ERR_MALLOC_FAIL;
    }
    let ret = unsafe { libc::pthread_spin_init(spin_tmp, 0) };
    if ret != 0 {
        let tag = b"osal_spinlock\0".as_ptr() as *const i8;
        let fmt = b"pthread_spin_init fail %d %d\0".as_ptr() as *const i8;
        let log_type = LOG_CORE as ::core::ffi::c_uint;
        let log_level = LOG_ERROR as ::core::ffi::c_uint;
        unsafe {
            HiLogPrint(log_type, log_level, 0xD002510u32, tag, fmt, ret, 36i32);
        }
        crate::src_osal_mem::OsalMemFree(spin_tmp as *mut ::core::ffi::c_void);
        return HDF_FAILURE;
    }
    unsafe {
        (*spinlock).realSpinlock = spin_tmp as *mut ::core::ffi::c_void;
    }
    HDF_SUCCESS
}

pub extern "C" fn OsalSpinDestroy(spinlock: *mut OsalSpinlock) -> i32 {
    if spinlock.is_null() || unsafe { (*spinlock).realSpinlock.is_null() } {
        let tag = b"osal_spinlock\0".as_ptr() as *const i8;
        let fmt = b"%s invalid param\0".as_ptr() as *const i8;
        let func = b"OsalSpinDestroy\0".as_ptr() as *const i8;
        let log_type = LOG_CORE as ::core::ffi::c_uint;
        let log_level = LOG_ERROR as ::core::ffi::c_uint;
        unsafe {
            HiLogPrint(
                log_type,
                log_level,
                0xD002510u32,
                tag,
                fmt,
                func,
            );
        }
        return HDF_ERR_INVALID_PARAM;
    }
    let real: *mut ::core::ffi::c_void = unsafe { (*spinlock).realSpinlock };
    let ret: i32 = unsafe { pthread_spin_destroy(real as *mut i32) };
    if ret != 0 {
        let tag = b"osal_spinlock\0".as_ptr() as *const i8;
        let fmt = b"pthread_spin_destroy fail %d %d\0".as_ptr() as *const i8;
        let log_type = LOG_CORE as ::core::ffi::c_uint;
        let log_level = LOG_ERROR as ::core::ffi::c_uint;
        unsafe {
            HiLogPrint(
                log_type,
                log_level,
                0xD002510u32,
                tag,
                fmt,
                ret,
                56i32,
            );
        }
        return HDF_FAILURE;
    }
    crate::src_osal_mem::OsalMemFree(real);
    unsafe {
        (*spinlock).realSpinlock = std::ptr::null_mut();
    }
    HDF_SUCCESS
}

pub extern "C" fn OsalSpinLock(spinlock: *mut OsalSpinlock) -> i32 {
    if spinlock.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    let real_spinlock = unsafe { (*spinlock).realSpinlock };
    if real_spinlock.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    let ret = unsafe { libc::pthread_spin_lock(real_spinlock as *mut libc::pthread_spinlock_t) };
    if ret != 0 {
        return HDF_FAILURE;
    }
    HDF_SUCCESS
}

pub extern "C" fn OsalSpinUnlock(spinlock: *mut OsalSpinlock) -> i32 {
    if spinlock.is_null() {
        let func = b"OsalSpinUnlock\0".as_ptr() as *const ::core::ffi::c_char;
        let tag = b"osal_spinlock\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"%s invalid param\0".as_ptr() as *const ::core::ffi::c_char;
        let log_core = crate::types::LOG_CORE as ::core::ffi::c_uint;
        let log_error = crate::types::LOG_ERROR as ::core::ffi::c_uint;
        unsafe {
            crate::compat::HiLogPrint(
                log_core,
                log_error,
                0xD002510u32,
                tag,
                fmt,
                func,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let real_spinlock = unsafe { (*spinlock).realSpinlock };
    if real_spinlock.is_null() {
        let func = b"OsalSpinUnlock\0".as_ptr() as *const ::core::ffi::c_char;
        let tag = b"osal_spinlock\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"%s invalid param\0".as_ptr() as *const ::core::ffi::c_char;
        let log_core = crate::types::LOG_CORE as ::core::ffi::c_uint;
        let log_error = crate::types::LOG_ERROR as ::core::ffi::c_uint;
        unsafe {
            crate::compat::HiLogPrint(
                log_core,
                log_error,
                0xD002510u32,
                tag,
                fmt,
                func,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let ret = unsafe {
        crate::compat::pthread_spin_unlock(
            real_spinlock as *mut i32,
        )
    };

    if ret != 0 {
        let func = b"OsalSpinUnlock\0".as_ptr() as *const ::core::ffi::c_char;
        let tag = b"osal_spinlock\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"pthread_spin_unlock fail %d %d\0".as_ptr() as *const ::core::ffi::c_char;
        let log_core = crate::types::LOG_CORE as ::core::ffi::c_uint;
        let log_error = crate::types::LOG_ERROR as ::core::ffi::c_uint;
        unsafe {
            crate::compat::HiLogPrint(
                log_core,
                log_error,
                0xD002510u32,
                tag,
                fmt,
                ret,
                95i32,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    crate::types::HDF_SUCCESS
}
