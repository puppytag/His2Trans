pub extern "C" fn HdfServiceObserverPublishService(observer: *mut crate::types::HdfServiceObserver, svcName: *const ::core::ffi::c_char, deviceId: crate::types::devid_t, policy: u16, service: *mut crate::types::HdfObject) -> ::core::ffi::c_int {
    let mut serviceRecord: *mut crate::types::HdfServiceObserverRecord = std::ptr::null_mut();
    let serviceKey = unsafe { crate::compat::HdfStringMakeHashKey(svcName, 0) };
    if observer.is_null() || svcName.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"service_observer\0".as_ptr() as *const _, b"observer or svcName is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    serviceRecord = unsafe { crate::compat::HdfSListSearch(&(*observer).services, serviceKey, Some(crate::src_hdf_observer_record::HdfServiceObserverRecordCompare)) as *mut crate::types::HdfServiceObserverRecord };
    if serviceRecord.is_null() {
        serviceRecord = crate::src_hdf_observer_record::HdfServiceObserverRecordObtain(serviceKey);
        if serviceRecord.is_null() {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"service_observer\0".as_ptr() as *const _, b"PublishService failed, serviceRecord is null\0".as_ptr() as *const _) };
            return crate::types::HDF_FAILURE;
        }
        unsafe {
            (*serviceRecord).publisher = service;
            (*serviceRecord).devId = deviceId;
            (*serviceRecord).policy = policy;
            let _ = crate::compat::OsalMutexLock(&mut (*observer).observerMutex);
            crate::compat::HdfSListAdd(&mut (*observer).services, &mut (*serviceRecord).entry);
            let _ = crate::compat::OsalMutexUnlock(&mut (*observer).observerMutex);
        }
    } else {
        unsafe {
            (*serviceRecord).publisher = service;
            crate::src_hdf_observer_record::HdfServiceObserverRecordNotifySubscribers(serviceRecord, deviceId, policy);
        }
    }
    crate::types::HDF_SUCCESS
}