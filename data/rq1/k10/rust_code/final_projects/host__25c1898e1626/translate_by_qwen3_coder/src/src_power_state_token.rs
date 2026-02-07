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
    let mut stateToken: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    if sref.is_null() {
        return;
    }
    unsafe {
        let __mptr = sref as *const crate::types::HdfSRef;
        let offset = std::mem::offset_of!(crate::types::PowerStateToken, wakeRef) as isize;
        stateToken = (__mptr as *mut u8).offset(-offset) as *mut crate::types::PowerStateToken;
    }
}

fn PowerStateTokenOnLastRelease(sref: *mut crate::types::HdfSRef) {
    let mut stateToken: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    let mut listener: *const crate::types::IPowerEventListener = std::ptr::null();
    if sref.is_null() {
        return;
    }
    unsafe {
        stateToken = (sref as *mut u8).offset(-(std::mem::offset_of!(crate::types::PowerStateToken, wakeRef) as isize)) as *mut crate::types::PowerStateToken;
        listener = (*stateToken).listener;
        if !listener.is_null() && (*listener).Suspend.is_some() {
            let _ = ((*listener).Suspend.unwrap())((*stateToken).deviceObject);
        }
        (*stateToken).psmState = crate::types::PSM_STATE_IDLE;
    }
}

pub extern "C" fn PowerStateChange(stateToken: *mut crate::types::PowerStateToken, pEvent: u32) -> ::core::ffi::c_int {
    if stateToken.is_null() {
        return crate::types::HDF_SUCCESS;
    }
    unsafe {
        if (*stateToken).listener.is_null() || (*stateToken).mode != crate::types::HDF_POWER_SYS_CTRL {
            return crate::types::HDF_SUCCESS;
        }
        match pEvent {
            crate::types::POWER_STATE_SUSPEND => {
                if let Some(f) = (*(*stateToken).listener).Suspend {
                    return f((*stateToken).deviceObject);
                }
            }
            crate::types::POWER_STATE_RESUME => {
                if let Some(f) = (*(*stateToken).listener).Resume {
                    return f((*stateToken).deviceObject);
                }
            }
            crate::types::POWER_STATE_DOZE_SUSPEND => {
                if let Some(f) = (*(*stateToken).listener).DozeSuspend {
                    return f((*stateToken).deviceObject);
                }
            }
            crate::types::POWER_STATE_DOZE_RESUME => {
                if let Some(f) = (*(*stateToken).listener).DozeResume {
                    return f((*stateToken).deviceObject);
                }
            }
            _ => {}
        }
    }
    crate::types::HDF_SUCCESS
}

fn PowerStateTokenAcquireWakeLock(token: *mut crate::types::IPowerStateToken) {
    let stateToken = token as *mut crate::types::PowerStateToken;
    if stateToken.is_null() {
        return;
    }
    unsafe {
        if (*stateToken).mode != crate::types::HDF_POWER_DYNAMIC_CTRL {
            return;
        }
        let sref = &mut (*stateToken).wakeRef as *mut crate::types::HdfSRef;
        if !sref.is_null() {
            if let Some(f) = (*sref).Acquire {
                f(sref);
            }
        }
    }
}

fn PowerStateTokenReleaseWakeLock(token: *mut crate::types::IPowerStateToken) {
    let stateToken = token as *mut crate::types::PowerStateToken;
    if stateToken.is_null() {
        return;
    }
    let mode = unsafe { (*stateToken).mode };
    if mode != crate::types::HDF_POWER_DYNAMIC_CTRL as u32 {
        return;
    }
    let sref = unsafe { &mut (*stateToken).wakeRef } as *mut crate::types::HdfSRef;
    if sref.is_null() {
        return;
    }
    let release_fn = unsafe { (*sref).Release };
    if release_fn.is_none() {
        return;
    }
    let count = unsafe { crate::compat::HdfSRefCount(sref as *const crate::types::HdfSRef) };
    if count == 0 {
        crate::src_power_state_token::PowerStateTokenOnLastRelease(sref);
    } else {
        unsafe { release_fn.unwrap()(sref) };
    }
}

fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    unsafe {
        let tokenIf = &mut (*powerStateToken).super_;
        let srefListener = OsalMemCalloc(std::mem::size_of::<crate::types::IHdfSRefListener>() as u32) as *mut crate::types::IHdfSRefListener;
        if srefListener.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }
        (*tokenIf).AcquireWakeLock = Some(core::mem::transmute(PowerStateTokenAcquireWakeLock as *const ()));
        (*tokenIf).ReleaseWakeLock = Some(core::mem::transmute(PowerStateTokenReleaseWakeLock as *const ()));
        (*srefListener).OnFirstAcquire = Some(core::mem::transmute(PowerStateTokenOnFirstAcquire as *const ()));
        (*srefListener).OnLastRelease = Some(core::mem::transmute(PowerStateTokenOnLastRelease as *const ()));
        (*powerStateToken).psmState = crate::types::PSM_STATE_IDLE;
        (*powerStateToken).listener = listener;
        (*powerStateToken).deviceObject = deviceObject;
        HdfSRefConstruct(&mut (*powerStateToken).wakeRef, srefListener);
        crate::types::HDF_SUCCESS
    }
}

pub extern "C" fn PowerStateTokenNewInstance(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> *mut crate::types::PowerStateToken {
    let stateToken = unsafe { libc::malloc(std::mem::size_of::<crate::types::PowerStateToken>()) as *mut crate::types::PowerStateToken };
    if stateToken.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        std::ptr::write_bytes(stateToken as *mut u8, 0, std::mem::size_of::<crate::types::PowerStateToken>());
    }
    let result = crate::src_power_state_token::PowerStateTokenConstruct(stateToken, deviceObject, listener);
    if result != crate::types::HDF_SUCCESS {
        unsafe {
            libc::free(stateToken as *mut libc::c_void);
        }
        return std::ptr::null_mut();
    }
    stateToken
}

pub extern "C" fn PowerStateTokenFreeInstance(stateToken: *mut crate::types::PowerStateToken) {
    if !stateToken.is_null() {
        unsafe {
            if !(*stateToken).wakeRef.listener.is_null() {
                crate::compat::OsalMemFree((*stateToken).wakeRef.listener as *mut ::core::ffi::c_void);
                (*stateToken).wakeRef.listener = std::ptr::null_mut();
            }
            crate::compat::OsalMemFree(stateToken as *mut ::core::ffi::c_void);
        }
    }
}
