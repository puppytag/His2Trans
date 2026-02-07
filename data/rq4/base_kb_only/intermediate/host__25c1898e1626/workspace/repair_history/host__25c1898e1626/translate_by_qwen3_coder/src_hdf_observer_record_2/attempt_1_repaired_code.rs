pub extern "C" fn HdfServiceObserverRecordRecycle(observerRecord: *mut crate::types::HdfServiceObserverRecord) {
    if !observerRecord.is_null() {
        unsafe {
            crate::compat::HdfSListFlush(
                &mut (*observerRecord).subscribers as *mut crate::types::HdfSList,
                Some(crate::src_hdf_observer_record::HdfServiceSubscriberDelete),
            );
            crate::compat::OsalMutexDestroy(
                &mut (*observerRecord).obsRecMutex as *mut crate::types::OsalMutex,
            );
            (*observerRecord).obsRecMutex.realMutex = std::ptr::null_mut();
            crate::compat::OsalMemFree(observerRecord as *mut ::core::ffi::c_void);
        }
    }
}