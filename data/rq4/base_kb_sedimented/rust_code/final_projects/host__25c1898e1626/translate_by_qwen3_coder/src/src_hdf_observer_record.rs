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
    let observerRecord = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::HdfServiceObserverRecord>() as u32)
    } as *mut crate::types::HdfServiceObserverRecord;
    
    if !observerRecord.is_null() {
        unsafe {
            (*observerRecord).serviceKey = serviceKey;
            (*observerRecord).publisher = std::ptr::null_mut();
            
            if crate::compat::OsalMutexInit(&mut (*observerRecord).obsRecMutex) != crate::types::HDF_SUCCESS {
                crate::compat::OsalMemFree(observerRecord as *mut ::core::ffi::c_void);
                return std::ptr::null_mut();
            }
            
            crate::compat::HdfSListInit(&mut (*observerRecord).subscribers);
        }
    }
    
    observerRecord
}

pub extern "C" fn HdfServiceObserverRecordRecycle(observerRecord: *mut crate::types::HdfServiceObserverRecord) {
    if !observerRecord.is_null() {
        unsafe {
            crate::compat::HdfSListFlush(
                &mut (*observerRecord).subscribers as *mut crate::types::HdfSList,
                Some(crate::src_hdf_service_subscriber::HdfServiceSubscriberDelete),
            );
            crate::compat::OsalMutexDestroy(
                &mut (*observerRecord).obsRecMutex as *mut crate::types::OsalMutex,
            );
            (*observerRecord).obsRecMutex.realMutex = std::ptr::null_mut();
            crate::compat::OsalMemFree(observerRecord as *mut ::core::ffi::c_void);
        }
    }
}

pub extern "C" fn HdfServiceObserverRecordCompare(listEntry: *mut crate::types::HdfSListNode, serviceKey: u32) -> bool {
    if listEntry.is_null() {
        return false;
    }
    let record = listEntry as *mut crate::types::HdfServiceObserverRecord;
    unsafe {
        if (*record).serviceKey == serviceKey {
            return true;
        }
    }
    false
}

pub extern "C" fn HdfServiceObserverRecordNotifySubscribers(record: *mut crate::types::HdfServiceObserverRecord, deviceId: crate::types::devid_t, policy: u16) {
    use crate::compat::*;
    use crate::types::*;
    
    if record.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD002510,
                b"observer_record\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: record is null\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfServiceObserverRecordNotifySubscribers\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    let mut it: HdfSListIterator = unsafe { std::mem::zeroed() };
    
    unsafe {
        OsalMutexLock(&mut (*record).obsRecMutex);
        HdfSListIteratorInit(&mut it, &(*record).subscribers as *const HdfSList);
        
        while HdfSListIteratorHasNext(&it as *const HdfSListIterator) {
            let subscriber = HdfSListIteratorNext(&mut it) as *mut HdfServiceSubscriber;
            if !subscriber.is_null() {
                if deviceId == (*subscriber).devId || policy != SERVICE_POLICY_PRIVATE as u16 {
                    (*subscriber).state = HDF_SUBSCRIBER_STATE_READY;
                    if let Some(on_connected) = (*subscriber).callback.OnServiceConnected {
                        on_connected((*subscriber).callback.deviceObject, (*record).publisher);
                    }
                }
            }
        }
        
        OsalMutexUnlock(&mut (*record).obsRecMutex);
    }
}

pub extern "C" fn HdfServiceObserverRecordDelete(listEntry: *mut crate::types::HdfSListNode) {
    let observerRecord = listEntry as *mut crate::types::HdfServiceObserverRecord;
    if !observerRecord.is_null() {
        crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(observerRecord);
    }
}
