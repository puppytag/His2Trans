//! Module: src_osal_time
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

pub extern "C" fn OsalGetTime(time: *mut OsalTimespec) -> i32 {
    if time.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let mut ts: libc::timespec = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe {
        libc::clock_gettime(1, &mut ts);
    }
    let sec = ts.tv_sec as u64;
    let usec = (ts.tv_nsec as u64) / 1000;
    unsafe {
        (*time).sec = sec;
        (*time).usec = usec;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn OsalDiffTime(start: *const OsalTimespec, end: *const OsalTimespec, diff: *mut OsalTimespec) -> i32 {
    if start.is_null() || end.is_null() || diff.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }

    let start_sec;
    let start_usec;
    let end_sec;
    let end_usec;

    unsafe {
        start_sec = (*start).sec;
        start_usec = (*start).usec;
        end_sec = (*end).sec;
        end_usec = (*end).usec;
    }

    let mut usec: u64 = 0;
    let mut sec: u64 = 0;

    if start_sec > end_sec || (end_sec == start_sec && end_usec < start_usec) {
        return HDF_ERR_INVALID_PARAM;
    }

    if end_usec < start_usec {
        usec = 1_000_000;
        sec = 1;
    }

    if start_sec > u64::MAX - sec
        || end_usec > u64::MAX - usec
        || end_sec < start_sec + sec
        || end_usec + usec < start_usec
    {
        return HDF_ERR_INVALID_PARAM;
    }

    let usec_diff = usec + end_usec - start_usec;
    let sec_diff = end_sec - start_sec - sec;
    unsafe {
        (*diff).usec = usec_diff;
        (*diff).sec = sec_diff;
    }

    HDF_SUCCESS
}

pub extern "C" fn OsalSleep(sec: u32) {
    let mut req = libc::timespec {
        tv_sec: sec as libc::time_t,
        tv_nsec: 0,
    };
    loop {
        let mut rem: libc::timespec = libc::timespec { tv_sec: 0, tv_nsec: 0 };
        let ret = unsafe { libc::nanosleep(&req, &mut rem) };
        if ret == 0 {
            break;
        }
        let errno = unsafe { *libc::__errno_location() };
        if errno != libc::EINTR {
            // For unexpected errors, return early (log omitted as not required).
            break;
        }
        req = rem;
    }
}

pub extern "C" fn OsalMSleep(ms: u32) {
    let mut req = libc::timespec {
        tv_sec: (ms / 1000) as libc::time_t,
        tv_nsec: ((ms % 1000) * 1_000_000) as libc::c_long,
    };
    loop {
        let mut rem: libc::timespec = libc::timespec { tv_sec: 0, tv_nsec: 0 };
        let ret = unsafe { libc::nanosleep(&req, &mut rem) };
        if ret == 0 {
            break;
        }
        let errno = unsafe { *libc::__errno_location() };
        if errno != libc::EINTR {
            // For unexpected errors, return early (log omitted as not required).
            break;
        }
        req = rem;
    }
}

pub extern "C" fn OsalUSleep(us: u32) {
    let ts = libc::timespec {
        tv_sec: (us / 1_000_000) as libc::time_t,
        tv_nsec: ((us % 1_000_000) * 1000) as libc::c_long,
    };
    unsafe {
        libc::nanosleep(&ts, std::ptr::null_mut());
    }
}

pub extern "C" fn OsalUDelay(us: u32) {
    let _ = us;
}

pub extern "C" fn OsalMDelay(ms: u32) {
    let _ = ms;
}

pub extern "C" fn OsalGetSysTimeMs() -> u64 {
    let mut time = crate::types::OsalTimespec { sec: 0, usec: 0 };
    let _ = crate::src_osal_time::OsalGetTime(&mut time as *mut _);
    time.sec * 1000 + time.usec / 1000
}
