pub extern "C" fn HdfServiceObserverRemoveRecord(observer: *mut crate::types::HdfServiceObserver, svcName: *const ::core::ffi::c_char) {
    let mut serviceRecord: *mut crate::types::HdfServiceObserverRecord = std::ptr::null_mut();
    if observer.is_null() || svcName.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_WARN, 0xD002510, b"service_observer\0".as_ptr() as *const _, b"observer or svcName is null\0".as_ptr() as *const _) };
        return;
    }
    let serviceKey = unsafe { crate::compat::HdfStringMakeHashKey(svcName, 0) };
    serviceRecord = unsafe { crate::compat::HdfSListSearch(&(*observer).services, serviceKey, Some(crate::compat::HdfServiceObserverRecordCompare)) as *mut crate::types::HdfServiceObserverRecord };
    if !serviceRecord.is_null() {
        unsafe {
            let _ = crate::compat::OsalMutexLock(&mut (*observer).observerMutex);
            crate::compat::HdfSListRemove(&mut (*observer).services, &mut (*serviceRecord).entry);
            let _ = crate::compat::OsalMutexUnlock(&mut (*observer).observerMutex);
            crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(serviceRecord);
        }
    }
}