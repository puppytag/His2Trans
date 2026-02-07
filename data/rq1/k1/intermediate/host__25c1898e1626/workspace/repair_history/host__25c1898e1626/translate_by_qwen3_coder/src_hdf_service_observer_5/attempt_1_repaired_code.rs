pub extern "C" fn HdfServiceObserverRemoveRecord(observer: *mut crate::types::HdfServiceObserver, svcName: *const ::core::ffi::c_char) {
    let mut serviceRecord: *mut crate::types::HdfServiceObserverRecord = std::ptr::null_mut();
    let serviceKey = unsafe { HdfStringMakeHashKey(svcName, 0) };
    if observer.is_null() || svcName.is_null() {
        let _ = unsafe { HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_WARN, 0xD002510, b"service_observer\0".as_ptr() as *const _, b"observer or svcName is null\0".as_ptr() as *const _) };
        return;
    }
    unsafe {
        serviceRecord = HdfSListSearch(&(*observer).services, serviceKey, Some(crate::src_hdf_observer_record::HdfServiceObserverRecordCompare)) as *mut crate::types::HdfServiceObserverRecord;
    }
    if !serviceRecord.is_null() {
        unsafe {
            OsalMutexLock(&mut (*observer).observerMutex);
            HdfSListRemove(&mut (*observer).services, &mut (*serviceRecord).entry);
            OsalMutexUnlock(&mut (*observer).observerMutex);
            crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(serviceRecord);
        }
    }
}