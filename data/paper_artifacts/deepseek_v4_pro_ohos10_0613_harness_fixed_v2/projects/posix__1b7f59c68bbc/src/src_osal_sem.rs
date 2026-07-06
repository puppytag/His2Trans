//! Module: src_osal_sem
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

pub extern "C" fn OsalSemInit(sem: *mut OsalSem, value: u32) -> i32 {
    if sem.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    unsafe {
        (*sem).realSemaphore = std::ptr::null_mut::<::core::ffi::c_void>();
    }

    let sem_tmp: *mut libc::sem_t = crate::src_osal_mem::OsalMemCalloc(
        std::mem::size_of::<libc::sem_t>() as crate::types::size_t,
    ) as *mut libc::sem_t;

    if sem_tmp.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    let pshared: i32 = unsafe { SHARE };
    let ret = unsafe { libc::sem_init(sem_tmp, pshared, value as libc::c_uint) };
    if ret != 0 {
        crate::src_osal_mem::OsalMemFree(sem_tmp as *mut ::core::ffi::c_void);
        return crate::types::HDF_FAILURE;
    }

    unsafe {
        (*sem).realSemaphore = sem_tmp as *mut ::core::ffi::c_void;
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn OsalSemWait(sem: *mut OsalSem, ms: u32) -> i32 {
    if sem.is_null() || unsafe { (*sem).realSemaphore.is_null() } {
        return HDF_ERR_INVALID_PARAM;
    }
    let sem_inner = unsafe { (*sem).realSemaphore as *mut libc::sem_t };
    if ms == 0xFFFF_FFFF_u32 {
        if unsafe { libc::sem_wait(sem_inner) != 0 } {
            return HDF_FAILURE;
        }
    } else {
        let mut time: libc::timespec = libc::timespec { tv_sec: 0, tv_nsec: 0 };
        unsafe { libc::clock_gettime(0, &mut time); }
        let ms_i64 = ms as i64;
        time.tv_sec += ms_i64 / 1000;
        time.tv_nsec += (ms_i64 % 1000) * 1000 * 1000;
        if time.tv_nsec >= 1_000_000_000 {
            time.tv_sec += 1;
            time.tv_nsec -= 1_000_000_000;
        }
        let ret = unsafe { libc::sem_timedwait(sem_inner, &time) };
        if ret != 0 {
            let errno_val = unsafe { *libc::__errno_location() };
            if errno_val == 110 {
                return HDF_ERR_TIMEOUT;
            } else {
                return HDF_FAILURE;
            }
        }
    }
    HDF_SUCCESS
}

pub extern "C" fn OsalSemPost(sem: *mut OsalSem) -> i32 {
    if sem.is_null() || unsafe { (*sem).realSemaphore.is_null() } {
        // HiLogPrint omitted due to unresolved symbol
        return HDF_ERR_INVALID_PARAM;
    }

    let real_sem = unsafe { (*sem).realSemaphore as *mut libc::sem_t };
    let ret = unsafe { libc::sem_post(real_sem) };
    if ret != 0 {
        // HiLogPrint with errno omitted due to unresolved symbols
        return HDF_FAILURE;
    }

    HDF_SUCCESS
}

pub extern "C" fn OsalSemDestroy(sem: *mut OsalSem) -> i32 {
    if sem.is_null() {
        let tag = b"osal_sem\0".as_ptr() as *const i8;
        let fmt = b"%s invalid param\0".as_ptr() as *const i8;
        let func = b"OsalSemDestroy\0".as_ptr() as *const i8;
        let log_type = LOG_CORE as crate::compat::LogType;
        let log_level = LOG_ERROR as crate::compat::LogLevel;
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

    let sem_ptr = unsafe { (*sem).realSemaphore };
    if sem_ptr.is_null() {
        let tag = b"osal_sem\0".as_ptr() as *const i8;
        let fmt = b"%s invalid param\0".as_ptr() as *const i8;
        let func = b"OsalSemDestroy\0".as_ptr() as *const i8;
        let log_type = LOG_CORE as crate::compat::LogType;
        let log_level = LOG_ERROR as crate::compat::LogLevel;
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

    let ret: i32 = unsafe { libc::sem_destroy(sem_ptr as *mut libc::sem_t) };
    if ret != 0 {
        let errno_val: i32 = unsafe { *__errno_location() };
        let tag = b"osal_sem\0".as_ptr() as *const i8;
        let fmt = b"sem_destroy fail errno:%d\0".as_ptr() as *const i8;
        let log_type = LOG_CORE as crate::compat::LogType;
        let log_level = LOG_ERROR as crate::compat::LogLevel;
        unsafe {
            HiLogPrint(
                log_type,
                log_level,
                0xD002510u32,
                tag,
                fmt,
                errno_val,
            );
        }
        return HDF_FAILURE;
    }

    crate::src_osal_mem::OsalMemFree(sem_ptr);
    unsafe {
        (*sem).realSemaphore = ::core::ptr::null_mut();
    }
    HDF_SUCCESS
}
