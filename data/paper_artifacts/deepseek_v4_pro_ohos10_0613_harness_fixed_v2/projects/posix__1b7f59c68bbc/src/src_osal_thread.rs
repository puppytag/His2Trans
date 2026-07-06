//! Module: src_osal_thread
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

pub const OSAL_PTHREAD_STACK_MIN: crate::types::size_t = 4096;

fn OsalThreadRemapSched(priority: i32, param: *mut crate::types::sched_param) {
    const OSAL_PRIORITY_LOW: i32 = 10;
    const OSAL_PRIORITY_MIDDLE: i32 = 50;
    const OSAL_PRIORITY_HIGH: i32 = 90;
    const OSAL_PRIORITY_HIGHEST: i32 = 99;

    let sched_pri = if priority == (crate::types::OSAL_THREAD_PRI_HIGHEST as i32) {
        OSAL_PRIORITY_HIGHEST
    } else if priority == (crate::types::OSAL_THREAD_PRI_HIGH as i32) {
        OSAL_PRIORITY_HIGH
    } else if priority == (crate::types::OSAL_THREAD_PRI_DEFAULT as i32) {
        OSAL_PRIORITY_MIDDLE
    } else {
        OSAL_PRIORITY_LOW
    };
    unsafe { (*param).sched_priority = sched_pri; }
}

pub extern "C" fn OsalThreadCreate(thread: *mut OsalThread, threadEntry: OsalThreadEntry, entryPara: *mut ::core::ffi::c_void) -> i32 {
    if thread.is_null() || threadEntry.is_none() {
        let tag = b"osal_thread\0" as *const u8 as *const i8;
        let fmt = b"%s invalid param\0" as *const u8 as *const i8;
        let func = b"OsalThreadCreate\0" as *const u8 as *const i8;
        let log_core = LOG_CORE as LogType;
        let log_error = LOG_ERROR as LogLevel;
        let _ = unsafe {
            HiLogPrint(
                log_core,
                log_error,
                0xD002510u32,
                tag,
                fmt,
                func,
            )
        };
        return HDF_ERR_INVALID_PARAM;
    }

    unsafe {
        (*thread).realThread = std::ptr::null_mut();
    }

    let para: *mut ThreadWrapper = crate::src_osal_mem::OsalMemCalloc(core::mem::size_of::<ThreadWrapper>() as crate::types::size_t) as *mut ThreadWrapper;

    if para.is_null() {
        let tag = b"osal_thread\0" as *const u8 as *const i8;
        let fmt = b"%s malloc fail\0" as *const u8 as *const i8;
        let func = b"OsalThreadCreate\0" as *const u8 as *const i8;
        let log_core = LOG_CORE as LogType;
        let log_error = LOG_ERROR as LogLevel;
        let _ = unsafe {
            HiLogPrint(
                log_core,
                log_error,
                0xD002510u32,
                tag,
                fmt,
                func,
            )
        };
        return HDF_FAILURE;
    }

    unsafe {
        (*para).entryPara = entryPara;
        (*para).threadEntry = threadEntry;
        (*thread).realThread = para as *mut ::core::ffi::c_void;
    }

    HDF_SUCCESS
}

pub extern "C" fn OsalThreadDestroy(thread: *mut OsalThread) -> i32 {
    if thread.is_null() {
        return crate::types::HDF_SUCCESS;
    }
    let real_thread = unsafe { (*thread).realThread };
    if !real_thread.is_null() {
        crate::src_osal_mem::OsalMemFree(real_thread);
        unsafe {
            (*thread).realThread = core::ptr::null_mut();
        }
    }
    crate::types::HDF_SUCCESS
}

fn OsalCreatePthread(threadId: *mut crate::types::pthread_t, attribute: *mut crate::types::pthread_attr_t, para: *mut crate::types::ThreadWrapper, name: *mut std::ffi::c_char)-> i32 {
    let libc_thread_id: *mut libc::pthread_t = threadId as *mut libc::pthread_t;
    let libc_attr: *const libc::pthread_attr_t = attribute as *const libc::pthread_attr_t;

    let thread_entry = unsafe { (*para).threadEntry };
    let entry_para = unsafe { (*para).entryPara };

    let start_routine: extern "C" fn(*mut std::ffi::c_void) -> *mut std::ffi::c_void =
        unsafe { std::mem::transmute(thread_entry) };

    let mut result_code = unsafe {
        libc::pthread_create(
            libc_thread_id,
            libc_attr,
            start_routine,
            entry_para,
        )
    };
    if result_code != 0 {
        return result_code;
    }

    if !name.is_null() {
        let mut thread_name: [std::ffi::c_char; 16] = [0; 16];
        let name_len = unsafe { libc::strlen(name) };
        let copy_len = if name_len >= 15 { 15 } else { name_len };
        if copy_len > 0 {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    name as *const u8,
                    thread_name.as_mut_ptr() as *mut u8,
                    copy_len as usize,
                );
            }
        }
        unsafe {
            libc::pthread_setname_np(*libc_thread_id, thread_name.as_ptr());
        }
    }

    result_code = unsafe { libc::pthread_detach(*libc_thread_id) };
    if result_code != 0 {
        return result_code;
    }

    result_code = unsafe { libc::pthread_attr_destroy(attribute as *mut libc::pthread_attr_t) };
    if result_code != 0 {
        return result_code;
    }

    crate::types::HDF_SUCCESS
}

fn OsalThreadSetSchedPolicy(attribute: *mut crate::types::pthread_attr_t, policy: i32)-> i32 {
    if policy == 0 {
        return crate::types::HDF_SUCCESS;
    }
    if policy != 1 && policy != 2 {
        // HiLogPrint omitted: unresolved symbol
        return crate::types::HDF_FAILURE;
    }
    let mut result_code: i32;
    unsafe {
        result_code = libc::pthread_attr_setinheritsched(attribute as *mut libc::pthread_attr_t, 1);
    }
    if result_code < 0 {
        // HiLogPrint omitted
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        result_code = libc::pthread_attr_setschedpolicy(attribute as *mut libc::pthread_attr_t, policy);
    }
    if result_code < 0 {
        // HiLogPrint omitted
        return crate::types::HDF_FAILURE;
    }
    crate::types::HDF_SUCCESS
}

pub extern "C" fn OsalThreadStart(thread: *mut OsalThread, param: *const OsalThreadParam) -> i32 {
    if thread.is_null() || param.is_null() || unsafe { (*thread).realThread.is_null() } {
        let tag = b"osal_thread\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"OsalThreadStart invalid param\0".as_ptr() as *const ::core::ffi::c_char;
        let log_core = LOG_CORE as LogType;
        let log_error = LOG_ERROR as LogLevel;
        unsafe {
            let _ = HiLogPrint(
                log_core,
                log_error,
                0xD002510u32,
                tag,
                fmt,
            );
        }
        return HDF_ERR_INVALID_PARAM;
    }

    let mut attribute: crate::types::pthread_attr_t = unsafe { ::std::mem::zeroed() };
    let mut priorityHolder: crate::types::sched_param = unsafe { ::std::mem::zeroed() };
    let para: *mut crate::types::ThreadWrapper = unsafe { (*thread).realThread as *mut crate::types::ThreadWrapper };

    let mut resultCode: i32;

    let final_result = loop {
        resultCode = unsafe { pthread_attr_init(&mut attribute) };
        if resultCode != 0 {
            let tag = b"osal_thread\0".as_ptr() as *const ::core::ffi::c_char;
            let fmt = b"pthread_attr_init error\0".as_ptr() as *const ::core::ffi::c_char;
            let log_core = LOG_CORE as LogType;
            let log_error = LOG_ERROR as LogLevel;
            unsafe {
                let _ = HiLogPrint(
                    log_core,
                    log_error,
                    0xD002510u32,
                    tag,
                    fmt,
                );
            }
            break HDF_FAILURE;
        }

        let mut stackSize: crate::types::size_t = unsafe { (*param).stackSize };
        if stackSize > 0 as crate::types::size_t {
            if stackSize < OSAL_PTHREAD_STACK_MIN {
                stackSize = OSAL_PTHREAD_STACK_MIN;
            }
            resultCode = unsafe { pthread_attr_setstacksize(&mut attribute, stackSize) };
            if resultCode != 0 {
                let tag = b"osal_thread\0".as_ptr() as *const ::core::ffi::c_char;
                let fmt = b"pthread_attr_setstacksize error\0".as_ptr() as *const ::core::ffi::c_char;
                let log_core = LOG_CORE as LogType;
                let log_error = LOG_ERROR as LogLevel;
                unsafe {
                    let _ = HiLogPrint(
                        log_core,
                        log_error,
                        0xD002510u32,
                        tag,
                        fmt,
                    );
                }
                break HDF_FAILURE;
            }
        }

        resultCode = unsafe { pthread_attr_getschedparam(&mut attribute, &mut priorityHolder) };
        if resultCode != 0 {
            let tag = b"osal_thread\0".as_ptr() as *const ::core::ffi::c_char;
            let fmt = b"pthread_attr_getschedparam error\0".as_ptr() as *const ::core::ffi::c_char;
            let log_core = LOG_CORE as LogType;
            let log_error = LOG_ERROR as LogLevel;
            unsafe {
                let _ = HiLogPrint(
                    log_core,
                    log_error,
                    0xD002510u32,
                    tag,
                    fmt,
                );
            }
            break HDF_FAILURE;
        }

        crate::src_osal_thread::OsalThreadRemapSched(unsafe { (*param).priority as i32 }, &mut priorityHolder);

        resultCode = unsafe { pthread_attr_setschedparam(&mut attribute, &mut priorityHolder) };
        if resultCode != 0 {
            let tag = b"osal_thread\0".as_ptr() as *const ::core::ffi::c_char;
            let fmt = b"pthread_attr_setschedparam error\0".as_ptr() as *const ::core::ffi::c_char;
            let log_core = LOG_CORE as LogType;
            let log_error = LOG_ERROR as LogLevel;
            unsafe {
                let _ = HiLogPrint(
                    log_core,
                    log_error,
                    0xD002510u32,
                    tag,
                    fmt,
                );
            }
            break HDF_FAILURE;
        }

        crate::src_osal_thread::OsalThreadSetSchedPolicy(&mut attribute, unsafe { (*param).policy });

        let name_arg = unsafe { (*param).name as *mut ::std::ffi::c_char };
        resultCode = crate::src_osal_thread::OsalCreatePthread(
            &mut unsafe { (*para).id } as *mut crate::types::pthread_t,
            &mut attribute as *mut crate::types::pthread_attr_t,
            para,
            name_arg,
        );
        if resultCode != 0 {
            let tag = b"osal_thread\0".as_ptr() as *const ::core::ffi::c_char;
            let fmt = b"OsalCreatePthread error\0".as_ptr() as *const ::core::ffi::c_char;
            let log_core = LOG_CORE as LogType;
            let log_error = LOG_ERROR as LogLevel;
            unsafe {
                let _ = HiLogPrint(
                    log_core,
                    log_error,
                    0xD002510u32,
                    tag,
                    fmt,
                );
            }
            break HDF_FAILURE;
        }

        break HDF_SUCCESS;
    };

    if final_result != HDF_SUCCESS {
        crate::src_osal_thread::OsalThreadDestroy(thread);
        return HDF_FAILURE;
    }
    HDF_SUCCESS
}
