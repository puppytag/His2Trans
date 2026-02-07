pub extern "C" fn HdfServiceObserverSubscribeService(
    observer: *mut crate::types::HdfServiceObserver,
    svcName: *const ::core::ffi::c_char,
    deviceId: crate::types::devid_t,
    callback: crate::types::SubscriberCallback,
) -> ::core::ffi::c_int {
    let mut serviceRecord: *mut crate::types::HdfServiceObserverRecord = std::ptr::null_mut();
    let mut subscriber: *mut crate::types::HdfServiceSubscriber = std::ptr::null_mut();
    let serviceKey = unsafe { crate::compat::HdfStringMakeHashKey(svcName, 0) };
    if observer.is_null() || svcName.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                b"observer or svcName or callback.OnServiceConnected is null\0".as_ptr()
                    as *const ::core::ffi::c_char,
            )
        };
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        serviceRecord = crate::compat::HdfSListSearch(
            &(*observer).services,
            serviceKey,
            crate::compat::HdfServiceObserverRecordCompare,
        ) as *mut crate::types::HdfServiceObserverRecord;
    }
    if serviceRecord.is_null() {
        serviceRecord = crate::src_hdf_observer_record::HdfServiceObserverRecordObtain(serviceKey);
        if serviceRecord.is_null() {
            let _ = unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD002510,
                    b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failed to subscribe service, serviceRecord is null\0".as_ptr()
                        as *const ::core::ffi::c_char,
                )
            };
            return crate::types::HDF_FAILURE;
        }
        subscriber =
            crate::src_hdf_service_subscriber::HdfServiceSubscriberObtain(callback, deviceId);
        if subscriber.is_null() {
            let _ = unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD002510,
                    b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failed to subscribe service, subscriber is null\0".as_ptr()
                        as *const ::core::ffi::c_char,
                )
            };
            crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(serviceRecord);
            return crate::types::HDF_FAILURE;
        }
        unsafe {
            let _ = crate::compat::OsalMutexLock(&mut (*observer).observerMutex);
            crate::compat::HdfSListAdd(&mut (*observer).services, &mut (*serviceRecord).entry);
            let _ = crate::compat::OsalMutexUnlock(&mut (*observer).observerMutex);
        }
    } else {
        subscriber =
            crate::src_hdf_service_subscriber::HdfServiceSubscriberObtain(callback, deviceId);
        if subscriber.is_null() {
            let _ = unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD002510,
                    b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                    b"failed to subscribe service, subscriber obtain null\0".as_ptr()
                        as *const ::core::ffi::c_char,
                )
            };
            return crate::types::HDF_FAILURE;
        }
    }
    unsafe {
        if !(*serviceRecord).publisher.is_null()
            && (*subscriber).callback.OnServiceConnected.is_some()
            && ((*serviceRecord).policy as u32 != crate::types::SERVICE_POLICY_PRIVATE
                || (*serviceRecord).devId == deviceId)
        {
            (*subscriber).state = crate::types::HDF_SUBSCRIBER_STATE_READY as u32;
            if let Some(f) = (*subscriber).callback.OnServiceConnected {
                f(
                    (*subscriber).callback.deviceObject,
                    (*serviceRecord).publisher as *const crate::types::HdfObject,
                );
            }
        }
    }
    unsafe {
        let _ = crate::compat::OsalMutexLock(&mut (*serviceRecord).obsRecMutex);
        crate::compat::HdfSListAdd(&mut (*serviceRecord).subscribers, &mut (*subscriber).entry);
        let _ = crate::compat::OsalMutexUnlock(&mut (*serviceRecord).obsRecMutex);
    }
    crate::types::HDF_SUCCESS
}