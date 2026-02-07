//! Module: src_hdf_service_observer
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

pub extern "C" fn HdfServiceObserverConstruct(observer: *mut crate::types::HdfServiceObserver) -> bool {
    if observer.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                b"observer is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return false;
    }
    unsafe {
        if crate::compat::OsalMutexInit(&mut (*observer).observerMutex) != crate::types::HDF_SUCCESS {
            return false;
        }
        crate::compat::HdfSListInit(&mut (*observer).services);
    }
    true
}

// === C2R MANUAL FIX REQUIRED ===
// reason: repair_failed_after_5
// func_key: src_hdf_service_observer_2
// c_function: HdfServiceObserverDestruct
// rust_file: src_hdf_service_observer.rs
// rust_signature: pub extern "C" fn HdfServiceObserverDestruct(observer: *mut crate::types::HdfServiceObserver)
// c_first_line: void HdfServiceObserverDestruct(struct HdfServiceObserver *observer)
// saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_service_observer_2/translated_rust.rs
// last_error_truncated:
//   error[E0425]: cannot find value `HdfServiceObserverRecordDelete` in the crate root
//     --> src/src_hdf_service_observer.rs:42:29
//      |
//      |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in the crate root
//      |
//   help: consider importing this function
//      |
//      |
// =================================
pub extern "C" fn HdfServiceObserverDestruct(observer: *mut crate::types::HdfServiceObserver) {
    // C2R: C2Rust fallback (LLM failed; wrapper keeps skeleton signature)
    unsafe { crate::compat::__c2rust_fallback::src_hdf_service_observer::HdfServiceObserverDestruct(observer as _); }
}
/* === C2R_LLM_FAILED_OUTPUT_BEGIN ===
 * func_key: src_hdf_service_observer_2
 * reason: repair_failed_after_5
 * saved_translation: /data/home/wangshb/c2-rust_framework/translation_outputs/claude-ohos5_B_sed/intermediate/host__25c1898e1626/workspace/repair_history/host__25c1898e1626/translate_by_qwen3_coder/_manual_fix/src_hdf_service_observer_2/translated_rust.rs
 * ------------------------------------------------------------
pub extern "C" fn HdfServiceObserverDestruct(observer: *mut crate::types::HdfServiceObserver) {
    if !observer.is_null() {
        unsafe {
            crate::compat::HdfSListFlush(
                &mut (*observer).services as *mut crate::types::HdfSList,
                Some(crate::HdfServiceObserverRecordDelete),
            );
            crate::compat::OsalMutexDestroy(
                &mut (*observer).observerMutex as *mut crate::types::OsalMutex,
            );
        }
    }
}
 * ------------------------------------------------------------
 * C2R_LLM_FAILED_OUTPUT_BEGIN func_key: src_hdf_service_observer_2
 * === C2R_LLM_FAILED_OUTPUT_END === */


pub extern "C" fn HdfServiceObserverSubscribeService(observer: *mut crate::types::HdfServiceObserver, svcName: *const ::core::ffi::c_char, deviceId: crate::types::devid_t, callback: crate::types::SubscriberCallback) -> ::core::ffi::c_int {
    let mut serviceRecord: *mut crate::types::HdfServiceObserverRecord = std::ptr::null_mut();
    let mut subscriber: *mut crate::types::HdfServiceSubscriber = std::ptr::null_mut();
    
    let serviceKey = unsafe { crate::compat::HdfStringMakeHashKey(svcName, 0) };
    
    if observer.is_null() || svcName.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                b"observer or svcName or callback.OnServiceConnected is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    
    serviceRecord = unsafe {
        crate::compat::HdfSListSearch(
            &mut (*observer).services as *mut crate::types::HdfSList,
            serviceKey,
            Some(crate::src_hdf_observer_record::HdfServiceObserverRecordCompare),
        ) as *mut crate::types::HdfServiceObserverRecord
    };
    
    if serviceRecord.is_null() {
        serviceRecord = crate::src_hdf_observer_record::HdfServiceObserverRecordObtain(serviceKey);
        if serviceRecord.is_null() {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failed to subscribe service, serviceRecord is null\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return crate::types::HDF_FAILURE;
        }
        subscriber = crate::src_hdf_service_subscriber::HdfServiceSubscriberObtain(callback, deviceId);
        if subscriber.is_null() {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failed to subscribe service, subscriber is null\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(serviceRecord);
            return crate::types::HDF_FAILURE;
        }
        unsafe {
            crate::compat::OsalMutexLock(&mut (*observer).observerMutex as *mut crate::types::OsalMutex);
            crate::compat::HdfSListAdd(&mut (*observer).services as *mut crate::types::HdfSList, &mut (*serviceRecord).entry as *mut crate::types::HdfSListNode);
            crate::compat::OsalMutexUnlock(&mut (*observer).observerMutex as *mut crate::types::OsalMutex);
        }
    } else {
        subscriber = crate::src_hdf_service_subscriber::HdfServiceSubscriberObtain(callback, deviceId);
        if subscriber.is_null() {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failed to subscribe service, subscriber obtain null\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return crate::types::HDF_FAILURE;
        }
    }
    
    unsafe {
        if !(*serviceRecord).publisher.is_null() && (*subscriber).callback.OnServiceConnected.is_some() &&
            (((*serviceRecord).policy as u32 != crate::types::SERVICE_POLICY_PRIVATE) || ((*serviceRecord).devId == deviceId)) {
            (*subscriber).state = crate::types::HDF_SUBSCRIBER_STATE_READY;
            if let Some(on_connected) = (*subscriber).callback.OnServiceConnected {
                on_connected((*subscriber).callback.deviceObject, (*serviceRecord).publisher);
            }
        }
        
        crate::compat::OsalMutexLock(&mut (*serviceRecord).obsRecMutex as *mut crate::types::OsalMutex);
        crate::compat::HdfSListAdd(&mut (*serviceRecord).subscribers as *mut crate::types::HdfSList, &mut (*subscriber).entry as *mut crate::types::HdfSListNode);
        crate::compat::OsalMutexUnlock(&mut (*serviceRecord).obsRecMutex as *mut crate::types::OsalMutex);
    }
    
    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfServiceObserverPublishService(observer: *mut crate::types::HdfServiceObserver, svcName: *const ::core::ffi::c_char, deviceId: crate::types::devid_t, policy: u16, service: *mut crate::types::HdfObject) -> ::core::ffi::c_int {
    let mut serviceRecord: *mut crate::types::HdfServiceObserverRecord = std::ptr::null_mut();
    let serviceKey: u32 = unsafe { crate::compat::HdfStringMakeHashKey(svcName, 0) };
    
    if observer.is_null() || svcName.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                b"observer or svcName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    
    serviceRecord = unsafe {
        crate::compat::HdfSListSearch(
            &mut (*observer).services as *mut crate::types::HdfSList,
            serviceKey,
            Some(crate::src_hdf_observer_record::HdfServiceObserverRecordCompare),
        ) as *mut crate::types::HdfServiceObserverRecord
    };
    
    if serviceRecord.is_null() {
        serviceRecord = crate::src_hdf_observer_record::HdfServiceObserverRecordObtain(serviceKey);
        if serviceRecord.is_null() {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                    b"PublishService failed, serviceRecord is null\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return crate::types::HDF_FAILURE;
        }
        unsafe {
            (*serviceRecord).publisher = service;
            (*serviceRecord).devId = deviceId;
            (*serviceRecord).policy = policy;
            crate::compat::OsalMutexLock(&mut (*observer).observerMutex as *mut crate::types::OsalMutex);
            crate::compat::HdfSListAdd(
                &mut (*observer).services as *mut crate::types::HdfSList,
                &mut (*serviceRecord).entry as *mut crate::types::HdfSListNode,
            );
            crate::compat::OsalMutexUnlock(&mut (*observer).observerMutex as *mut crate::types::OsalMutex);
        }
    } else {
        unsafe {
            (*serviceRecord).publisher = service;
        }
        crate::src_hdf_observer_record::HdfServiceObserverRecordNotifySubscribers(serviceRecord, deviceId, policy);
    }
    
    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfServiceObserverRemoveRecord(observer: *mut crate::types::HdfServiceObserver, svcName: *const ::core::ffi::c_char) {
    use crate::compat::*;
    
    let mut serviceRecord: *mut crate::types::HdfServiceObserverRecord = std::ptr::null_mut();
    let serviceKey: u32 = unsafe { HdfStringMakeHashKey(svcName, 0) };
    
    if observer.is_null() || svcName.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_WARN,
                0xD002510,
                b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                b"observer or svcName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    serviceRecord = unsafe {
        HdfSListSearch(
            &mut (*observer).services as *mut crate::types::HdfSList,
            serviceKey,
            Some(crate::src_hdf_observer_record::HdfServiceObserverRecordCompare),
        ) as *mut crate::types::HdfServiceObserverRecord
    };
    
    if !serviceRecord.is_null() {
        unsafe {
            OsalMutexLock(&mut (*observer).observerMutex as *mut crate::types::OsalMutex);
            HdfSListRemove(
                &mut (*observer).services as *mut crate::types::HdfSList,
                &mut (*serviceRecord).entry as *mut crate::types::HdfSListNode,
            );
            OsalMutexUnlock(&mut (*observer).observerMutex as *mut crate::types::OsalMutex);
            crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(serviceRecord);
        }
    }
}
