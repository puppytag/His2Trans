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

pub(crate) struct HdfSListIter {
    next: *mut crate::types::HdfSListNode,
}

impl HdfSListIter {
    pub(crate) fn new(head: *mut crate::types::HdfSListNode) -> Self {
        HdfSListIter { next: head }
    }
}

impl Iterator for HdfSListIter {
    type Item = *mut crate::types::HdfSListNode;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        }
        let current = self.next;
        // SAFETY: current is non-null and a valid HdfSListNode.
        self.next = unsafe { (*current).next };
        Some(current)
    }
}

pub extern "C" fn HdfServiceObserverRecordObtain(serviceKey: u32) -> *mut crate::types::HdfServiceObserverRecord {
    let observerRecord = unsafe {
        libc::calloc(1, std::mem::size_of::<crate::types::HdfServiceObserverRecord>())
            as *mut crate::types::HdfServiceObserverRecord
    };
    if observerRecord.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*observerRecord).serviceKey = serviceKey;
        (*observerRecord).publisher = std::ptr::null_mut();
    }
    let mutex_init_ok = unsafe {
        OsalMutexInit(&mut (*observerRecord).obsRecMutex as *mut crate::types::OsalMutex) == crate::types::HDF_SUCCESS
    };
    if !mutex_init_ok {
        unsafe { libc::free(observerRecord as *mut ::core::ffi::c_void); }
        return std::ptr::null_mut();
    }
    unsafe {
        HdfSListInit(&mut (*observerRecord).subscribers as *mut crate::types::HdfSList);
    }
    observerRecord
}

pub extern "C" fn HdfServiceObserverRecordRecycle(observerRecord: *mut crate::types::HdfServiceObserverRecord) {
    if observerRecord.is_null() {
        return;
    }

    // Flush subscribers: iterate the HdfSList, calling HdfServiceSubscriberDelete for each node.
    {
        // SAFETY: observerRecord is non-null, subscribers.root is valid.
        let head = unsafe { (*observerRecord).subscribers.root };
        for node in HdfSListIter::new(head) {
            unsafe {
                crate::src_hdf_service_subscriber::HdfServiceSubscriberDelete(node);
            }
        }
    }

    // Destroy the mutex associated with this observer record.
    unsafe {
        let _ = OsalMutexDestroy(&mut (*observerRecord).obsRecMutex);
    }

    // The mutex is destroyed; clear the internal pointer as the C code does.
    unsafe {
        (*observerRecord).obsRecMutex.realMutex = std::ptr::null_mut();
    }

    // Free the observer record memory.
    unsafe {
        OsalMemFree(observerRecord as *mut ::core::ffi::c_void);
    }
}

pub extern "C" fn HdfServiceObserverRecordCompare(listEntry: *mut crate::types::HdfSListNode, serviceKey: u32) -> bool {
    if listEntry.is_null() {
        return false;
    }
    let record = listEntry as *mut crate::types::HdfServiceObserverRecord;
    unsafe { (*record).serviceKey == serviceKey }
}

pub extern "C" fn HdfServiceObserverRecordNotifySubscribers(record: *mut crate::types::HdfServiceObserverRecord, deviceId: crate::types::devid_t, policy: u16) {
    if record.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                b"observer_record\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: record is null\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfServiceObserverRecordNotifySubscribers\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }

    // Lock mutex
    unsafe {
        let mutex_ptr = ::core::ptr::addr_of_mut!((*record).obsRecMutex);
        OsalMutexLock(mutex_ptr);
    }

    // Iterate subscribers using Rust HdfSListIter
    {
        let head = unsafe { (*record).subscribers.root };
        for node_ptr in HdfSListIter::new(head) {
            // SAFETY: node_ptr is a valid HdfSListNode; the subscriber starts with that node.
            let subscriber_ptr: *mut crate::types::HdfServiceSubscriber = node_ptr as *mut _;
            let subscriber = unsafe { &mut *subscriber_ptr };
            let subscriber_dev_id = subscriber.devId;
            if deviceId == subscriber_dev_id || policy as u32 != crate::types::SERVICE_POLICY_PRIVATE as u32 {
                subscriber.state = crate::types::HDF_SUBSCRIBER_STATE_READY;
                if let Some(callback) = subscriber.callback.OnServiceConnected {
                    // SAFETY: callback is a valid function pointer; arguments are valid.
                    let publisher_ptr = unsafe { (*record).publisher as *const crate::types::HdfObject };
                    unsafe { callback(subscriber.callback.deviceObject, publisher_ptr); }
                }
            }
        }
    }

    // Unlock mutex
    unsafe {
        let mutex_ptr = ::core::ptr::addr_of_mut!((*record).obsRecMutex);
        OsalMutexUnlock(mutex_ptr);
    }
}

pub extern "C" fn HdfServiceObserverRecordDelete(listEntry: *mut crate::types::HdfSListNode) {
    let observerRecord = listEntry as *mut crate::types::HdfServiceObserverRecord;
    if !observerRecord.is_null() {
        unsafe {
            crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(observerRecord);
        }
    }
}
