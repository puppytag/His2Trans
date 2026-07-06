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
// Source: archived_runs/deepseek-v4-pro-ohos10-full-0613-1/raw/framework_output/intermediate/manager__c248934e0221/workspace/.preprocessed/*.i (preprocessed translation units)
// Details: archived_runs/deepseek-v4-pro-ohos10-full-0613-1/raw/framework_output/intermediate/manager__c248934e0221/workspace/repair_history/manager__c248934e0221/translate_by_qwen3_coder/_manual_fix/extern_decls_from_bindgen_allowlist.json
#[allow(improper_ctypes)]
#[allow(non_snake_case)]
extern "C" {
    pub fn DevHostServiceFreeInstance(service: *mut crate::types::IDevHostService);
    pub fn DevHostServiceNewInstance(hostId: u16, hostName: *const ::core::ffi::c_char) -> *mut crate::types::IDevHostService;
    pub fn DevSvcRecordFreeInstance(inst: *mut crate::types::DevSvcRecord);
    pub fn DevSvcRecordNewInstance() -> *mut crate::types::DevSvcRecord;
    pub fn DeviceManagerIsQuickLoad() -> ::core::ffi::c_int;
    pub fn HdfAttributeManagerGetDeviceList(hostClnt: *mut crate::types::DevHostServiceClnt) -> ::core::ffi::c_int;
    pub fn HdfAttributeManagerGetHostList(hostList: *mut crate::types::HdfSList) -> bool;
    pub fn HdfDeviceSendEventToClient(client: *const crate::types::HdfDeviceIoClient, id: u32, data: *const crate::types::HdfSBuf) -> i32;
    pub fn HdfIoServicePublish(serviceName: *const ::core::ffi::c_char, mode: u32) -> *mut crate::types::HdfIoService;
    pub fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService);
    pub fn HdfObjectManagerGetObject(objectId: ::core::ffi::c_int) -> *mut crate::types::HdfObject;
    pub fn HdfSListAdd(list: *mut crate::types::HdfSList, link: *mut crate::types::HdfSListNode);
    pub fn HdfSListCount(list: *const crate::types::HdfSList) -> ::core::ffi::c_int;
    pub fn HdfSListFlush(list: *mut crate::types::HdfSList, deleter: crate::types::HdfSListDeleter);
    pub fn HdfSListInit(list: *mut crate::types::HdfSList);
    pub fn HdfSListIsEmpty(list: *const crate::types::HdfSList) -> bool;
    pub fn HdfSListIteratorHasNext(iterator: *const crate::types::HdfSListIterator) -> bool;
    pub fn HdfSListIteratorInit(iterator: *mut crate::types::HdfSListIterator, list: *const crate::types::HdfSList);
    pub fn HdfSListIteratorNext(iterator: *mut crate::types::HdfSListIterator) -> *mut crate::types::HdfSListNode;
    pub fn HdfSListIteratorRemove(iterator: *mut crate::types::HdfSListIterator);
    pub fn HdfSListRemove(list: *mut crate::types::HdfSList, link: *mut crate::types::HdfSListNode);
    pub fn HdfSListSearch(list: *const crate::types::HdfSList, keyValue: u32, comparer: crate::types::HdfSListSearchComparer) -> *mut crate::types::HdfSListNode;
    pub fn HdfSbufFlush(sbuf: *mut crate::types::HdfSBuf);
    pub fn HdfSbufObtainDefaultSize() -> *mut crate::types::HdfSBuf;
    pub fn HdfSbufReadUint16(sbuf: *mut crate::types::HdfSBuf, value: *mut u16) -> bool;
    pub fn HdfSbufReadUint64(sbuf: *mut crate::types::HdfSBuf, value: *mut u64) -> bool;
    pub fn HdfSbufRecycle(sbuf: *mut crate::types::HdfSBuf);
    pub fn HdfSbufWriteString(sbuf: *mut crate::types::HdfSBuf, value: *const ::core::ffi::c_char) -> bool;
    pub fn HdfSbufWriteUint16(sbuf: *mut crate::types::HdfSBuf, value: u16) -> bool;
    pub fn HdfSbufWriteUint32(sbuf: *mut crate::types::HdfSBuf, value: u32) -> bool;
    pub fn HdfStringCopy(src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    pub fn HdfStringMakeHashKey(key: *const ::core::ffi::c_char, mask: u32) -> u32;
    pub fn HdfSysEventSend(eventClass: u64, event: u32, content: *const ::core::ffi::c_char, sync: bool) -> i32;
    pub fn HiLogPrint(type_: crate::types::LogType, level: crate::types::LogLevel, domain: ::core::ffi::c_uint, tag: *const ::core::ffi::c_char, fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    pub fn MapInit(map: *mut crate::types::Map);
    pub fn OsalMSleep(ms: u32);
    pub fn OsalMemCalloc(size: crate::types::size_t) -> *mut ::core::ffi::c_void;
    pub fn OsalMemFree(mem: *mut ::core::ffi::c_void);
    pub fn OsalMutexDestroy(mutex: *mut crate::types::OsalMutex) -> i32;
    pub fn OsalMutexInit(mutex: *mut crate::types::OsalMutex) -> i32;
    pub fn OsalMutexLock(mutex: *mut crate::types::OsalMutex) -> i32;
    pub fn OsalMutexUnlock(mutex: *mut crate::types::OsalMutex) -> i32;
    pub fn ServiceStatusMarshalling(status: *mut crate::types::ServiceStatus, buf: *mut crate::types::HdfSBuf) -> ::core::ffi::c_int;
    pub fn strcmp(arg1: *const ::core::ffi::c_char, arg2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
// === C2R_EXTERN_DECLS_END ===


/// Stable replacement for `core::mem::offset_of!`.
/// Computes the byte offset of a field within a struct by using `ptr::addr_of!` on a null pointer.
macro_rules! offset_of {
    ($Struct:ty, $field:ident) => {{
        let null: *const $Struct = ::core::ptr::null();
        let field_ptr = unsafe { ::core::ptr::addr_of!((*null).$field) };
        (field_ptr as usize) - (null as usize)
    }};
}
pub(crate) use offset_of;


