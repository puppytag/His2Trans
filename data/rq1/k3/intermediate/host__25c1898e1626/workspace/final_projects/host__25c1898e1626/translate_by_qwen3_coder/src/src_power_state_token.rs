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
        let offset = std::mem::offset_of!(crate::types::PowerStateToken, wakeRef) as isize;
        stateToken = (sref as *mut u8).offset(-offset) as *mut crate::types::PowerStateToken;
    }
}

fn PowerStateTokenOnLastRelease(sref: *mut crate::types::HdfSRef) {
    let mut stateToken: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    let mut listener: *const crate::types::IPowerEventListener = std::ptr::null();
    if sref.is_null() {
        return;
    }
    unsafe {
        let offset = std::mem::offset_of!(crate::types::PowerStateToken, wakeRef) as isize;
        stateToken = (sref as *mut u8).offset(-offset) as *mut crate::types::PowerStateToken;
        listener = (*stateToken).listener;
        if !listener.is_null() && (*listener).Suspend.is_some() {
            let _ = ((*listener).Suspend.unwrap())((*stateToken).deviceObject);
        }
        (*stateToken).psmState = crate::types::PSM_STATE_INACTIVE;
    }
}

pub extern "C" fn PowerStateChange(stateToken: *mut crate::types::PowerStateToken, pEvent: u32) -> ::core::ffi::c_int {
    if stateToken.is_null() {
        return crate::types::HDF_SUCCESS;
    }
    unsafe {
        let token = &*stateToken;
        if token.listener.is_null() || token.mode != crate::types::HDF_POWER_SYS_CTRL {
            return crate::types::HDF_SUCCESS;
        }
        let listener = &*token.listener;
        match pEvent {
            crate::types::POWER_STATE_SUSPEND => {
                if let Some(f) = listener.Suspend {
                    return f(token.deviceObject);
                }
            }
            crate::types::POWER_STATE_RESUME => {
                if let Some(f) = listener.Resume {
                    return f(token.deviceObject);
                }
            }
            crate::types::POWER_STATE_DOZE_SUSPEND => {
                if let Some(f) = listener.DozeSuspend {
                    return f(token.deviceObject);
                }
            }
            crate::types::POWER_STATE_DOZE_RESUME => {
                if let Some(f) = listener.DozeResume {
                    return f(token.deviceObject);
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
    if mode != crate::types::HDF_POWER_DYNAMIC_CTRL {
        return;
    }
    let sref = unsafe { &mut (*stateToken).wakeRef as *mut crate::types::HdfSRef };
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

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_power_state_token_6
// c_function: PowerStateTokenConstruct
// rust_file: src_power_state_token.rs
// rust_signature: fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> i32
// c_first_line: static int32_t PowerStateTokenConstruct(struct PowerStateToken *powerStateToken,
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk3/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_power_state_token_6/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `PowerStateTokenAcquireWakeLock` in module `crate::compat`
//      --> src/src_power_state_token.rs:109:58
//       |
//       |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::compat`
//   error[E0425]: cannot find value `PowerStateTokenReleaseWakeLock` in module `crate::compat`
//      --> src/src_power_state_token.rs:110:58
//       |
//       |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::compat`
// =================================
fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> i32 {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_power_state_token::PowerStateTokenConstruct(powerStateToken as _, deviceObject as _, listener as _) as _ }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_power_state_token_6
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk3/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_power_state_token_6/translated_rust.rs
 * ------------------------------------------------------------
fn PowerStateTokenConstruct(powerStateToken: *mut crate::types::PowerStateToken, deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> i32 {
    unsafe {
        let tokenIf = &mut (*powerStateToken).super_ as *mut crate::types::IPowerStateToken;
        let srefListener = libc::calloc(1, std::mem::size_of::<crate::types::IHdfSRefListener>()) as *mut crate::types::IHdfSRefListener;
        if srefListener.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL;
        }
        (*tokenIf).AcquireWakeLock = Some(crate::compat::PowerStateTokenAcquireWakeLock);
        (*tokenIf).ReleaseWakeLock = Some(crate::compat::PowerStateTokenReleaseWakeLock);
        (*srefListener).OnFirstAcquire = Some(crate::compat::PowerStateTokenOnFirstAcquire);
        (*srefListener).OnLastRelease = Some(crate::compat::PowerStateTokenOnLastRelease);
        (*powerStateToken).psmState = crate::types::PSM_STATE_IDLE;
        (*powerStateToken).listener = listener;
        (*powerStateToken).deviceObject = deviceObject;
        crate::compat::HdfSRefConstruct(&mut (*powerStateToken).wakeRef, srefListener);
        crate::types::HDF_SUCCESS
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_power_state_token_6
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn PowerStateTokenNewInstance(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> *mut crate::types::PowerStateToken {
    let stateToken = unsafe { libc::calloc(1, std::mem::size_of::<crate::types::PowerStateToken>()) } as *mut crate::types::PowerStateToken;
    if stateToken.is_null() {
        return std::ptr::null_mut();
    }
    let result = crate::src_power_state_token::PowerStateTokenConstruct(stateToken, deviceObject, listener);
    if result != crate::types::HDF_SUCCESS {
        unsafe { libc::free(stateToken as *mut std::ffi::c_void) };
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
