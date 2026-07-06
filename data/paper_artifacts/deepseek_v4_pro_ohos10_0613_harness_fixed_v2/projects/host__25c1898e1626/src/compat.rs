//! Compatibility / Fallback Layer
//!
//! This module is auto-generated to keep the translated project compiling.
//!
//! Design goals:
//! - Centralize placeholders and shims in ONE place (easy to audit & remove later).
//! - Keep function bodies as close to translated semantics as possible.
//!
//! IMPORTANT:
//! - Items here may be placeholders (value/layout unknown). Always review before relying on semantics.

#![allow(dead_code)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

/// Minimal FFI prelude (for legacy skeletons that import `crate::compat::ffi::*`).
pub mod ffi {
    pub use core::ffi::*;
}

// === C2R_COMPAT_PLACEHOLDERS_BEGIN ===
// (auto-appended placeholders will be inserted here)
// === C2R_COMPAT_PLACEHOLDERS_END ===

// === C2R_EXTERN_DECLS_BEGIN ===
// Auto-generated extern decls (C2R step 2.55; bindgen allowlist).
// Source: archived_runs/deepseek-v4-pro-ohos10-full-0613-1/raw/framework_output/intermediate/host__25c1898e1626/workspace/.preprocessed/*.i (preprocessed translation units)
// Details: archived_runs/deepseek-v4-pro-ohos10-full-0613-1/raw/framework_output/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/extern_decls_from_bindgen_allowlist.json
#[allow(improper_ctypes)]
#[allow(non_snake_case)]
extern "C" {
    pub fn DevMgrPmRegister() -> ::core::ffi::c_int;
    pub fn HcsGetNodeByMatchAttr(node: *const crate::types::DeviceResourceNode, attrValue: *const ::core::ffi::c_char) -> *const crate::types::DeviceResourceNode;
    pub fn HdfDriverManagerGetDriver(driverName: *const ::core::ffi::c_char) -> *mut crate::types::HdfDriver;
    pub fn HdfGetHcsRootNode() -> *const crate::types::DeviceResourceNode;
    pub fn HdfObjectManagerFreeObject(object: *mut crate::types::HdfObject);
    pub fn HdfObjectManagerGetObject(objectId: ::core::ffi::c_int) -> *mut crate::types::HdfObject;
    pub fn HdfRegisterDriverEntry(entry: *const crate::types::HdfDriverEntry) -> i32;
    pub fn HdfSListAdd(list: *mut crate::types::HdfSList, link: *mut crate::types::HdfSListNode);
    pub fn HdfSListFlush(list: *mut crate::types::HdfSList, deleter: crate::types::HdfSListDeleter);
    pub fn HdfSListInit(list: *mut crate::types::HdfSList);
    pub fn HdfSListIteratorHasNext(iterator: *const crate::types::HdfSListIterator) -> bool;
    pub fn HdfSListIteratorInit(iterator: *mut crate::types::HdfSListIterator, list: *const crate::types::HdfSList);
    pub fn HdfSListIteratorNext(iterator: *mut crate::types::HdfSListIterator) -> *mut crate::types::HdfSListNode;
    pub fn HdfSListRemove(list: *mut crate::types::HdfSList, link: *mut crate::types::HdfSListNode);
    pub fn HdfSListSearch(list: *const crate::types::HdfSList, keyValue: u32, comparer: crate::types::HdfSListSearchComparer) -> *mut crate::types::HdfSListNode;
    pub fn HdfSRefConstruct(sref: *mut crate::types::HdfSRef, listener: *mut crate::types::IHdfSRefListener);
    pub fn HdfSRefCount(sref: *const crate::types::HdfSRef) -> ::core::ffi::c_int;
    pub fn HdfStringCopy(src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn HdfStringMakeHashKey(key: *const ::core::ffi::c_char, mask: u32) -> u32;
    pub fn HdfTaskEnqueue(queue: *mut crate::types::HdfTaskQueue, task: *mut crate::types::HdfTaskType);
    pub fn HdfTaskQueueCreate(func: crate::types::HdfTaskFunc, name: *const ::core::ffi::c_char) -> *mut crate::types::HdfTaskQueue;
    pub fn HdfTaskQueueDestroy(queue: *mut crate::types::HdfTaskQueue);
    pub fn HiLogPrint(type_: crate::types::LogType, level: crate::types::LogLevel, domain: ::core::ffi::c_uint, tag: *const ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn OsalMemCalloc(size: crate::types::size_t) -> *mut ::core::ffi::c_void;
    pub fn OsalMemFree(mem: *mut ::core::ffi::c_void);
    pub fn OsalMutexDestroy(mutex: *mut crate::types::OsalMutex) -> i32;
    pub fn OsalMutexInit(mutex: *mut crate::types::OsalMutex) -> i32;
    pub fn OsalMutexLock(mutex: *mut crate::types::OsalMutex) -> i32;
    pub fn OsalMutexUnlock(mutex: *mut crate::types::OsalMutex) -> i32;
    pub fn __errno_location() -> *mut ::core::ffi::c_int;
    pub fn dlclose(arg1: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn dlerror() -> *mut ::core::ffi::c_char;
    pub fn dlopen(arg1: *const ::core::ffi::c_char, arg2: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    pub fn dlsym(arg1: *mut ::core::ffi::c_void, arg2: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
    pub fn pthread_rwlock_init(arg1: *mut crate::types::pthread_rwlock_t, arg2: *const crate::types::pthread_rwlockattr_t) -> ::core::ffi::c_int;
    pub fn pthread_rwlock_unlock(arg1: *mut crate::types::pthread_rwlock_t) -> ::core::ffi::c_int;
    pub fn pthread_rwlock_wrlock(arg1: *mut crate::types::pthread_rwlock_t) -> ::core::ffi::c_int;
    pub fn realpath(arg1: *const ::core::ffi::c_char, arg2: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn snprintf_s(strDest: *mut ::core::ffi::c_char, destMax: crate::types::size_t, count: crate::types::size_t, format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn strlen(arg1: *const ::core::ffi::c_char) -> crate::types::size_t;
    pub fn strncmp(arg1: *const ::core::ffi::c_char, arg2: *const ::core::ffi::c_char, arg3: crate::types::size_t) -> ::core::ffi::c_int;
}
// === C2R_EXTERN_DECLS_END ===

// === C2R_ACCESSOR_SHIMS_BEGIN ===
// (auto-appended accessor shim declarations will be inserted here)
// === C2R_ACCESSOR_SHIMS_BEGIN ===
// (auto-appended accessor shim declarations will be inserted here)
// === C2R_ACCESSOR_SHIMS_END ===

/// RAII guard for OsalMutex: unlocks in Drop.  Obtain via unsafe fn osal_mutex_lock.
pub struct OsalMutexGuard {
    mutex: *mut crate::types::OsalMutex,
}

impl OsalMutexGuard {
    /// # Safety
    /// Caller must ensure `mutex` points to a valid, initialized OsalMutex that can be locked.
    pub unsafe fn new(mutex: *mut crate::types::OsalMutex) -> Self {
        crate::compat::OsalMutexLock(mutex);
        OsalMutexGuard { mutex }
    }
}

impl Drop for OsalMutexGuard {
    fn drop(&mut self) {
        unsafe { crate::compat::OsalMutexUnlock(self.mutex); }
    }
}

/// RAII guard for pthread_rwlock_t: acquires write lock on construction, unlocks on drop.
pub struct RwlockGuard {
    rwlock: *mut crate::types::pthread_rwlock_t,
}

impl RwlockGuard {
    /// # Safety
    /// Caller must ensure `rwlock` points to a valid, initialized pthread_rwlock_t.
    pub unsafe fn new(rwlock: *mut crate::types::pthread_rwlock_t) -> Self {
        crate::compat::pthread_rwlock_wrlock(rwlock);
        RwlockGuard { rwlock }
    }
}

impl Drop for RwlockGuard {
    fn drop(&mut self) {
        unsafe { crate::compat::pthread_rwlock_unlock(self.rwlock); }
    }
}

