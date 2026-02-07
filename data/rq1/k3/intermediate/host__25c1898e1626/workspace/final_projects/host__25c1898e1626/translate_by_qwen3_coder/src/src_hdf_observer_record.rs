//! Module: src_hdf_observer_record
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

pub extern "C" fn HdfServiceObserverRecordObtain(serviceKey: u32) -> *mut crate::types::HdfServiceObserverRecord {
    let observerRecord = unsafe { libc::calloc(1, std::mem::size_of::<crate::types::HdfServiceObserverRecord>()) } as *mut crate::types::HdfServiceObserverRecord;
    if !observerRecord.is_null() {
        unsafe {
            (*observerRecord).serviceKey = serviceKey;
            (*observerRecord).publisher = std::ptr::null_mut();
            if OsalMutexInit(&mut (*observerRecord).obsRecMutex) != crate::types::HDF_SUCCESS {
                libc::free(observerRecord as *mut ::core::ffi::c_void);
                return std::ptr::null_mut();
            }
            HdfSListInit(&mut (*observerRecord).subscribers);
        }
    }
    observerRecord
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_hdf_observer_record_2
// c_function: HdfServiceObserverRecordRecycle
// rust_file: src_hdf_observer_record.rs
// rust_signature: pub extern "C" fn HdfServiceObserverRecordRecycle(observerRecord: *mut crate::types::HdfServiceObserverRecord)
// c_first_line: void HdfServiceObserverRecordRecycle(struct HdfServiceObserverRecord *observerRecord)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk3/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_observer_record_2/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `HdfServiceSubscriberDelete` in the crate root
//     --> src/src_hdf_observer_record.rs:34:90
//      |
//      |                                                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in the crate root
//      |
//   help: consider importing this function
//      |
//      |
// =================================
pub extern "C" fn HdfServiceObserverRecordRecycle(observerRecord: *mut crate::types::HdfServiceObserverRecord) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_hdf_observer_record::HdfServiceObserverRecordRecycle(observerRecord as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_hdf_observer_record_2
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/deepseek-coder-ohos5_reuseS1_topk3/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_observer_record_2/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfServiceObserverRecordRecycle(observerRecord: *mut crate::types::HdfServiceObserverRecord) {
    if !observerRecord.is_null() {
        unsafe {
            crate::compat::HdfSListFlush(&mut (*observerRecord).subscribers, Some(crate::HdfServiceSubscriberDelete));
            let _ = crate::compat::OsalMutexDestroy(&mut (*observerRecord).obsRecMutex);
            (*observerRecord).obsRecMutex.realMutex = std::ptr::null_mut();
            crate::compat::OsalMemFree(observerRecord as *mut ::core::ffi::c_void);
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_hdf_observer_record_2
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn HdfServiceObserverRecordCompare(listEntry: *mut crate::types::HdfSListNode, serviceKey: u32) -> bool {
    if listEntry.is_null() {
        return false;
    }
    let record = listEntry as *mut crate::types::HdfServiceObserverRecord;
    unsafe { (*record).serviceKey == serviceKey }
}

pub extern "C" fn HdfServiceObserverRecordNotifySubscribers(record: *mut crate::types::HdfServiceObserverRecord, deviceId: crate::types::devid_t, policy: u16) {
    let mut it = crate::types::HdfSListIterator {
        stepOnNext: 0,
        prev: std::ptr::null_mut(),
        curr: std::ptr::null_mut(),
    };
    if record.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"observer_record\0".as_ptr() as *const _,
                b"%{public}s: record is null\0".as_ptr() as *const _,
                b"HdfServiceObserverRecordNotifySubscribers\0".as_ptr() as *const _,
            );
        }
        return;
    }
    unsafe {
        crate::compat::OsalMutexLock(&mut (*record).obsRecMutex);
        crate::compat::HdfSListIteratorInit(&mut it, &(*record).subscribers);
        while crate::compat::HdfSListIteratorHasNext(&it) {
            let subscriber = crate::compat::HdfSListIteratorNext(&mut it) as *mut crate::types::HdfServiceSubscriber;
            if deviceId == (*subscriber).devId || policy != crate::types::SERVICE_POLICY_PRIVATE as u16 {
                (*subscriber).state = crate::types::HDF_SUBSCRIBER_STATE_READY as u32;
                if let Some(cb) = (*subscriber).callback.OnServiceConnected {
                    cb((*subscriber).callback.deviceObject, (*record).publisher);
                }
            }
        }
        crate::compat::OsalMutexUnlock(&mut (*record).obsRecMutex);
    }
}

pub extern "C" fn HdfServiceObserverRecordDelete(listEntry: *mut crate::types::HdfSListNode) {
    let observerRecord = listEntry as *mut crate::types::HdfServiceObserverRecord;
    if !observerRecord.is_null() {
        crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(observerRecord);
    }
}
