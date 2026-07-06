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
            libc::printf(
                b"service_observer: observer is null\n\0".as_ptr() as *const ::core::ffi::c_char
            );
        }
        return false;
    }
    unsafe {
        if OsalMutexInit(
            &mut (*observer).observerMutex as *mut crate::types::OsalMutex
        ) != crate::types::HDF_SUCCESS
        {
            return false;
        }
        HdfSListInit(&mut (*observer).services as *mut crate::types::HdfSList);
    }
    true
}

pub extern "C" fn HdfServiceObserverDestruct(observer: *mut crate::types::HdfServiceObserver) {
    if observer.is_null() {
        return;
    }
    // SAFETY: observer is non-null; reading services.root yields a valid list head pointer
    let root_node = unsafe { (*observer).services.root };
    for node in crate::src_hdf_observer_record::HdfSListIter::new(root_node) {
        unsafe {
            crate::src_hdf_observer_record::HdfServiceObserverRecordDelete(node);
        }
    }
    unsafe {
        (*observer).services.root = std::ptr::null_mut();
        OsalMutexDestroy(&mut (*observer).observerMutex as *mut crate::types::OsalMutex);
    }
}

pub extern "C" fn HdfServiceObserverSubscribeService(observer: *mut crate::types::HdfServiceObserver, svcName: *const ::core::ffi::c_char, deviceId: crate::types::devid_t, callback: crate::types::SubscriberCallback) -> ::core::ffi::c_int {
    if observer.is_null() || svcName.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510u32,
                std::ffi::CStr::from_bytes_with_nul(b"service_observer\0").unwrap().as_ptr(),
                std::ffi::CStr::from_bytes_with_nul(b"observer or svcName or callback.OnServiceConnected is null\0").unwrap().as_ptr(),
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let serviceKey: u32 = unsafe { crate::compat::HdfStringMakeHashKey(svcName, 0) };
    let mut serviceRecord: *mut crate::types::HdfServiceObserverRecord = std::ptr::null_mut();
    let mut subscriber: *mut crate::types::HdfServiceSubscriber = std::ptr::null_mut();

    // Search or create service record
    serviceRecord = unsafe {
        let services_ptr = core::ptr::addr_of_mut!((*observer).services);
        crate::compat::HdfSListSearch(
            services_ptr as *mut crate::types::HdfSList,
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
                    0xD002510u32,
                    std::ffi::CStr::from_bytes_with_nul(b"service_observer\0").unwrap().as_ptr(),
                    std::ffi::CStr::from_bytes_with_nul(b"failed to subscribe service, serviceRecord is null\0").unwrap().as_ptr(),
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
                    0xD002510u32,
                    std::ffi::CStr::from_bytes_with_nul(b"service_observer\0").unwrap().as_ptr(),
                    std::ffi::CStr::from_bytes_with_nul(b"failed to subscribe service, subscriber is null\0").unwrap().as_ptr(),
                );
            }
            crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(serviceRecord);
            return crate::types::HDF_FAILURE;
        }

        // SAFETY: raw mutex pointer is valid; guard will unlock in Drop
        let _observer_guard = unsafe { crate::compat::OsalMutexGuard::new(core::ptr::addr_of_mut!((*observer).observerMutex)) };
        unsafe {
            let services_ptr = core::ptr::addr_of_mut!((*observer).services);
            crate::compat::HdfSListAdd(
                services_ptr,
                core::ptr::addr_of_mut!((*serviceRecord).entry),
            );
        }
    } else {
        subscriber = crate::src_hdf_service_subscriber::HdfServiceSubscriberObtain(callback, deviceId);
        if subscriber.is_null() {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510u32,
                    std::ffi::CStr::from_bytes_with_nul(b"service_observer\0").unwrap().as_ptr(),
                    std::ffi::CStr::from_bytes_with_nul(b"failed to subscribe service, subscriber obtain null\0").unwrap().as_ptr(),
                );
            }
            return crate::types::HDF_FAILURE;
        }
    }

    // Callback invocation if publisher exists
    let (publisher, on_service_connected, record_policy, record_dev_id) = unsafe {
        (
            (*serviceRecord).publisher,
            (*subscriber).callback.OnServiceConnected,
            (*serviceRecord).policy,
            (*serviceRecord).devId,
        )
    };
    let invoke_connected = !publisher.is_null()
        && on_service_connected.is_some()
        && (record_policy as u32 != crate::types::SERVICE_POLICY_PRIVATE
            || record_dev_id == deviceId);
    if invoke_connected {
        unsafe {
            (*subscriber).state = crate::types::HDF_SUBSCRIBER_STATE_READY as u32;
        }
        let on_connected = unsafe { (*subscriber).callback.OnServiceConnected };
        if let Some(on_connected) = on_connected {
            let publisher = unsafe { (*serviceRecord).publisher };
            let device_object = unsafe { (*subscriber).callback.deviceObject };
            unsafe {
                on_connected(
                    device_object,
                    publisher as *const crate::types::HdfObject,
                );
            }
        }
    }

    // SAFETY: raw mutex pointer is valid; guard unlocks in Drop
    let _rec_guard = unsafe { crate::compat::OsalMutexGuard::new(core::ptr::addr_of_mut!((*serviceRecord).obsRecMutex)) };
    unsafe {
        crate::compat::HdfSListAdd(
            core::ptr::addr_of_mut!((*serviceRecord).subscribers),
            core::ptr::addr_of_mut!((*subscriber).entry),
        );
    }

    crate::types::HDF_SUCCESS
}

pub extern "C" fn HdfServiceObserverPublishService(observer: *mut crate::types::HdfServiceObserver, svcName: *const ::core::ffi::c_char, deviceId: crate::types::devid_t, policy: u16, service: *mut crate::types::HdfObject) -> ::core::ffi::c_int {
    if observer.is_null() || svcName.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0x00D002510u32,
                b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                b"observer or svcName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }

    let serviceKey: u32 = unsafe { crate::compat::HdfStringMakeHashKey(svcName, 0) };
    let serviceRecord: *mut crate::types::HdfServiceObserverRecord = unsafe {
        crate::compat::HdfSListSearch(
            core::ptr::addr_of_mut!((*observer).services),
            serviceKey,
            Some(crate::src_hdf_observer_record::HdfServiceObserverRecordCompare),
        ) as *mut crate::types::HdfServiceObserverRecord
    };

    if serviceRecord.is_null() {
        let new_record = crate::src_hdf_observer_record::HdfServiceObserverRecordObtain(serviceKey);
        if new_record.is_null() {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0x00D002510u32,
                    b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                    b"PublishService failed, serviceRecord is null\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            return crate::types::HDF_FAILURE;
        }
        // Assign publisher fields (raw pointer writes)
        unsafe {
            (*new_record).publisher = service;
            (*new_record).devId = deviceId;
            (*new_record).policy = policy;
        }
        let _guard = unsafe { crate::compat::OsalMutexGuard::new(core::ptr::addr_of_mut!((*observer).observerMutex)) };
        unsafe {
            crate::compat::HdfSListAdd(core::ptr::addr_of_mut!((*observer).services), core::ptr::addr_of_mut!((*new_record).entry));
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
    if observer.is_null() || svcName.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_WARN,
                0xD002510u32,
                b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                b"observer or svcName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }

    let service_key: u32 = unsafe { HdfStringMakeHashKey(svcName, 0u32) };
    let (list_ptr, search_result) = unsafe {
        let list_ptr: *mut crate::types::HdfSList = &mut (*observer).services as *mut _;
        let search_result: *mut crate::types::HdfSListNode =
            HdfSListSearch(list_ptr as *const _, service_key, Some(crate::src_hdf_observer_record::HdfServiceObserverRecordCompare));
        (list_ptr, search_result)
    };
    let service_record: *mut crate::types::HdfServiceObserverRecord =
        search_result as *mut _;

    if !service_record.is_null() {
        let _guard = unsafe {
            OsalMutexGuard::new(core::ptr::addr_of_mut!((*observer).observerMutex))
        };
        unsafe {
            let entry_ptr: *mut crate::types::HdfSListNode =
                &mut (*service_record).entry as *mut _;
            HdfSListRemove(list_ptr, entry_ptr);
            crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(service_record);
        }
    }
}
