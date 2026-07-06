//! Module: src_osal_mutex
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

pub extern "C" fn OsalMutexInit(mutex: *mut OsalMutex) -> i32 {
    if mutex.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        (*mutex).realMutex = std::ptr::null_mut();
    }
    let size = std::mem::size_of::<crate::types::pthread_mutex_t>();
    let ptr = crate::src_osal_mem::OsalMemCalloc(size as crate::types::size_t);
    if ptr.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }
    let mutex_tmp: *mut crate::types::pthread_mutex_t = ptr as *mut crate::types::pthread_mutex_t;
    let ret = unsafe { pthread_mutex_init(mutex_tmp, std::ptr::null()) };
    if ret != 0 {
        crate::src_osal_mem::OsalMemFree(mutex_tmp as *mut ::core::ffi::c_void);
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        (*mutex).realMutex = mutex_tmp as *mut ::core::ffi::c_void;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn OsalMutexDestroy(mutex: *mut OsalMutex) -> i32 {
    if mutex.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    let real_mutex = unsafe { (*mutex).realMutex };
    if real_mutex.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    let ret = unsafe { pthread_mutex_destroy(real_mutex as *mut pthread_mutex_t) };
    if ret != 0 {
        // HiLogPrint omitted as unavailable
    }
    crate::src_osal_mem::OsalMemFree(real_mutex);
    unsafe {
        (*mutex).realMutex = std::ptr::null_mut();
    }
    HDF_SUCCESS
}

pub extern "C" fn OsalMutexLock(mutex: *mut OsalMutex) -> i32 {
    if mutex.is_null() || unsafe { (*mutex).realMutex.is_null() } {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    crate::src_osal_mutex::OsalMutexTimedLock(mutex, u32::MAX)
}

pub extern "C" fn OsalMutexTimedLock(mutex: *mut OsalMutex, ms: u32) -> i32 {
    if mutex.is_null() || unsafe { (*mutex).realMutex.is_null() } {
        let tag = b"osal_mutex\0".as_ptr() as *const i8;
        let fmt = b"%s invalid param\0".as_ptr() as *const i8;
        let func = b"OsalMutexTimedLock\0".as_ptr() as *const i8;
        let log_type = LOG_CORE as u32;
        let log_level = LOG_ERROR as u32;
        unsafe {
            let _ = HiLogPrint(
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

    let real_mutex = unsafe { (*mutex).realMutex as *mut pthread_mutex_t };

    if ms == 0xFFFFFFFFu32 {
        let ret = unsafe { pthread_mutex_lock(real_mutex) };
        if ret != 0 {
            let tag = b"osal_mutex\0".as_ptr() as *const i8;
            let fmt = b"pthread_mutex_lock fail %d\0".as_ptr() as *const i8;
            let log_type = LOG_CORE as u32;
            let log_level = LOG_ERROR as u32;
            unsafe {
                let _ = HiLogPrint(
                    log_type,
                    log_level,
                    0xD002510u32,
                    tag,
                    fmt,
                    ret,
                );
            }
            return HDF_FAILURE;
        }
    } else {
        let mut time = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        unsafe {
            clock_gettime(0, &mut time);
        }
        time.tv_sec += (ms / 1000) as time_t;
        time.tv_nsec += ((ms % 1000) as time_t) * 1000 * 1000;
        if time.tv_nsec >= 1000000000 {
            time.tv_nsec -= 1000000000;
            time.tv_sec += 1;
        }
        let ret = unsafe { pthread_mutex_timedlock(real_mutex, &time) };
        if ret != 0 {
            if ret == 110 {
                return HDF_ERR_TIMEOUT;
            } else {
                    let tag = b"osal_mutex\0".as_ptr() as *const i8;
                    let fmt = b"%s time_out time:%u ret:%d\0".as_ptr() as *const i8;
                    let func = b"OsalMutexTimedLock\0".as_ptr() as *const i8;
                    let log_type = LOG_CORE as u32;
                    let log_level = LOG_ERROR as u32;
                    unsafe {
                        let _ = HiLogPrint(
                            log_type,
                            log_level,
                            0xD002510u32,
                            tag,
                            fmt,
                            func,
                            ms,
                            ret,
                        );
                    }
                return HDF_FAILURE;
            }
        }
    }

    HDF_SUCCESS
}

pub extern "C" fn OsalMutexUnlock(mutex: *mut OsalMutex) -> i32 {
    if mutex.is_null() {
        let tag = "osal_mutex\0".as_ptr() as *const i8;
        let fmt = "%s invalid param\0".as_ptr() as *const i8;
        let func = "OsalMutexUnlock\0".as_ptr() as *const i8;
        let log_core: u32 = LOG_CORE.try_into().unwrap();
        let log_error: u32 = LOG_ERROR.try_into().unwrap();
        unsafe {
            let _ = HiLogPrint(
                log_core,
                log_error,
                0xD002510u32,
                tag,
                fmt,
                func,
            );
        }
        return HDF_ERR_INVALID_PARAM;
    }
    let real_mutex = unsafe { (*mutex).realMutex };
    if real_mutex.is_null() {
        let tag = "osal_mutex\0".as_ptr() as *const i8;
        let fmt = "%s invalid param\0".as_ptr() as *const i8;
        let func = "OsalMutexUnlock\0".as_ptr() as *const i8;
        let log_core: u32 = LOG_CORE.try_into().unwrap();
        let log_error: u32 = LOG_ERROR.try_into().unwrap();
        unsafe {
            let _ = HiLogPrint(
                log_core,
                log_error,
                0xD002510u32,
                tag,
                fmt,
                func,
            );
        }
        return HDF_ERR_INVALID_PARAM;
    }
    let ret = unsafe { libc::pthread_mutex_unlock(real_mutex as *mut libc::pthread_mutex_t) };
    if ret != 0 {
        let tag = "osal_mutex\0".as_ptr() as *const i8;
        let fmt = "%s failed to pthread unlock %d\0".as_ptr() as *const i8;
        let func = "OsalMutexUnlock\0".as_ptr() as *const i8;
        let log_core: u32 = LOG_CORE.try_into().unwrap();
        let log_error: u32 = LOG_ERROR.try_into().unwrap();
        unsafe {
            let _ = HiLogPrint(
                log_core,
                log_error,
                0xD002510u32,
                tag,
                fmt,
                func,
                ret,
            );
        }
    }
    ret
}
