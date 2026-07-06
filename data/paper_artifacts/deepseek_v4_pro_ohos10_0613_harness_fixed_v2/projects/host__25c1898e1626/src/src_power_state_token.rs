//! Module: src_power_state_token
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

fn PowerStateTokenOnFirstAcquire(sref: *mut crate::types::HdfSRef) {
    // null check and early return are safe
    if sref.is_null() {
        return;
    }
    // SAFETY: container_of is required to get the enclosing struct from the embedded field
    let stateToken = unsafe {
        let offset = ::core::ptr::addr_of!((*std::ptr::null::<crate::types::PowerStateToken>()).wakeRef) as isize;
        ((sref as *mut u8).offset(-(offset as isize))) as *mut crate::types::PowerStateToken
    };
    // SAFETY: stateToken derives from valid sref and must point to alive PowerStateToken
    let token = unsafe { &mut *stateToken };
    // psmState check and assignment are safe on &mut reference
    if token.psmState == crate::types::PSM_STATE_ACTIVE {
        return;
    }
    let listener = token.listener;
    if !listener.is_null() {
        // SAFETY: listener is non-null, IHdfSRefListener ABI is assumed valid
        let resume = unsafe { (*listener).Resume };
        if let Some(resume_fn) = resume {
            // SAFETY: callback invocation via function pointer
            unsafe { resume_fn(token.deviceObject); }
        }
    }
    token.psmState = crate::types::PSM_STATE_ACTIVE;
}

fn PowerStateTokenOnLastRelease(sref: *mut crate::types::HdfSRef) {
    if sref.is_null() {
        return;
    }

    // SAFETY: container_of via field offset computation
    let stateToken = unsafe {
        let field_offset = {
            let null_token = core::ptr::null::<crate::types::PowerStateToken>();
            core::ptr::addr_of!((*null_token).wakeRef) as usize
        };
        ((sref as usize) - field_offset) as *mut crate::types::PowerStateToken
    };

    // SAFETY: stateToken is valid, reading listener and deviceObject fields
    let listener = unsafe { (*stateToken).listener };
    if !listener.is_null() {
        if let Some(suspend_fn) = unsafe { (*listener).Suspend } {
            let device_obj = unsafe { (*stateToken).deviceObject };
            if !device_obj.is_null() {
                // SAFETY: callback invocation
                unsafe { suspend_fn(device_obj); }
            }
        }
    }

    // SAFETY: psmState write
    unsafe {
        (*stateToken).psmState = crate::types::PSM_STATE_IDLE;
    }
}

pub extern "C" fn PowerStateChange(stateToken: *mut crate::types::PowerStateToken, pEvent: u32) -> ::core::ffi::c_int {
    if stateToken.is_null() {
        return crate::types::HDF_SUCCESS;
    }
    let listener = unsafe { (*stateToken).listener };
    let mode = unsafe { (*stateToken).mode };
    if listener.is_null() || mode != crate::types::HDF_POWER_SYS_CTRL {
        return crate::types::HDF_SUCCESS;
    }
    let listener_ref = unsafe { &*listener };
    let device_obj = unsafe { (*stateToken).deviceObject };
    match pEvent {
        crate::types::POWER_STATE_SUSPEND => {
            if let Some(suspend_fn) = listener_ref.Suspend {
                return unsafe { suspend_fn(device_obj) };
            }
        }
        crate::types::POWER_STATE_RESUME => {
            if let Some(resume_fn) = listener_ref.Resume {
                return unsafe { resume_fn(device_obj) };
            }
        }
        crate::types::POWER_STATE_DOZE_SUSPEND => {
            if let Some(doze_suspend_fn) = listener_ref.DozeSuspend {
                return unsafe { doze_suspend_fn(device_obj) };
            }
        }
        crate::types::POWER_STATE_DOZE_RESUME => {
            if let Some(doze_resume_fn) = listener_ref.DozeResume {
                return unsafe { doze_resume_fn(device_obj) };
            }
        }
        _ => {}
    }
    crate::types::HDF_SUCCESS
}

fn PowerStateTokenAcquireWakeLock(token: *mut crate::types::IPowerStateToken) {
    let state_token = token as *mut crate::types::PowerStateToken;
    if state_token.is_null() {
        return;
    }
    // SAFETY: state_token is non-null; mode is a valid integer field
    if unsafe { (*state_token).mode } as crate::types::PowerManagementMode != crate::types::HDF_POWER_DYNAMIC_CTRL {
        return;
    }
    let sref: *mut crate::types::HdfSRef =
        unsafe { std::ptr::addr_of_mut!((*state_token).wakeRef) };
    if sref.is_null() {
        return;
    }
    // SAFETY: sref is non-null; raw function-pointer deref and callback call require unsafe
    unsafe {
        if let Some(acquire) = (*sref).Acquire {
            acquire(sref);
        }
    }
}

fn PowerStateTokenReleaseWakeLock(token: *mut crate::types::IPowerStateToken) {
    if token.is_null() {
        return;
    }
    let state_token = token as *mut crate::types::PowerStateToken;

    let mode = unsafe { (*state_token).mode };
    if mode != crate::types::HDF_POWER_DYNAMIC_CTRL as u32 {
        return;
    }

    let sref: *mut crate::types::HdfSRef = unsafe { &mut (*state_token).wakeRef };
    // sref is never null when state_token is valid, but keep the check for safety
    if sref.is_null() {
        return;
    }

    let release_field = unsafe { (*sref).Release };
    if release_field.is_none() {
        return;
    }
    let count_field = unsafe { (*sref).Count };
    let count = match count_field {
        Some(count_fn) => unsafe { count_fn(sref as *const crate::types::HdfSRef) },
        None => return,
    };

    if count == 0 {
        unsafe {
            crate::src_power_state_token::PowerStateTokenOnLastRelease(sref);
        }
    } else {
        let release_fn = release_field.unwrap();
        unsafe {
            release_fn(sref);
        }
    }
}

fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener)-> i32 {
    if powerStateToken.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let token = unsafe { &mut *powerStateToken };
    let tokenIf: *mut crate::types::IPowerStateToken = &mut token.super_;

    let srefListener: *mut crate::types::IHdfSRefListener = unsafe {
        libc::calloc(1, core::mem::size_of::<crate::types::IHdfSRefListener>()) as *mut crate::types::IHdfSRefListener
    };
    if srefListener.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    // Compute transmuted function pointers once
    let acquire_fn: unsafe extern "C" fn(*mut crate::types::IPowerStateToken) = unsafe {
        ::core::mem::transmute::<
            fn(*mut crate::types::IPowerStateToken),
            unsafe extern "C" fn(*mut crate::types::IPowerStateToken)
        >(crate::src_power_state_token::PowerStateTokenAcquireWakeLock)
    };
    let release_fn: unsafe extern "C" fn(*mut crate::types::IPowerStateToken) = unsafe {
        ::core::mem::transmute::<
            fn(*mut crate::types::IPowerStateToken),
            unsafe extern "C" fn(*mut crate::types::IPowerStateToken)
        >(crate::src_power_state_token::PowerStateTokenReleaseWakeLock)
    };
    let on_first_acquire_fn: unsafe extern "C" fn(*mut crate::types::HdfSRef) = unsafe {
        ::core::mem::transmute::<
            fn(*mut crate::types::HdfSRef),
            unsafe extern "C" fn(*mut crate::types::HdfSRef)
        >(crate::src_power_state_token::PowerStateTokenOnFirstAcquire)
    };
    let on_last_release_fn: unsafe extern "C" fn(*mut crate::types::HdfSRef) = unsafe {
        ::core::mem::transmute::<
            fn(*mut crate::types::HdfSRef),
            unsafe extern "C" fn(*mut crate::types::HdfSRef)
        >(crate::src_power_state_token::PowerStateTokenOnLastRelease)
    };

    // Assign vtable entries (raw pointer writes)
    unsafe {
        (*tokenIf).AcquireWakeLock = Some(acquire_fn);
        (*tokenIf).ReleaseWakeLock = Some(release_fn);
    }
    unsafe {
        (*srefListener).OnFirstAcquire = Some(on_first_acquire_fn);
        (*srefListener).OnLastRelease = Some(on_last_release_fn);
    }

    // Safe field assignments via mutable reference
    token.psmState = crate::types::PSM_STATE_IDLE;
    token.listener = listener;
    token.deviceObject = deviceObject;
    token.wakeRef.refs.counter = 1;
    token.wakeRef.listener = srefListener;

    crate::types::HDF_SUCCESS
}

pub extern "C" fn PowerStateTokenNewInstance(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> *mut crate::types::PowerStateToken {
    let stateToken = unsafe {
        libc::calloc(1, std::mem::size_of::<crate::types::PowerStateToken>())
            as *mut crate::types::PowerStateToken
    };
    if stateToken.is_null() {
        return std::ptr::null_mut();
    }

    if crate::src_power_state_token::PowerStateTokenConstruct(stateToken, deviceObject, listener)
        != crate::types::HDF_SUCCESS
    {
        unsafe { libc::free(stateToken as *mut ::core::ffi::c_void); }
        return std::ptr::null_mut();
    }

    stateToken
}

pub extern "C" fn PowerStateTokenFreeInstance(stateToken: *mut crate::types::PowerStateToken) {
    if stateToken.is_null() {
        return;
    }
    unsafe {
        let listener = (*stateToken).wakeRef.listener;
        if !listener.is_null() {
            OsalMemFree(listener as *mut ::core::ffi::c_void);
            (*stateToken).wakeRef.listener = ::core::ptr::null_mut();
        }
        OsalMemFree(stateToken as *mut ::core::ffi::c_void);
    }
}
