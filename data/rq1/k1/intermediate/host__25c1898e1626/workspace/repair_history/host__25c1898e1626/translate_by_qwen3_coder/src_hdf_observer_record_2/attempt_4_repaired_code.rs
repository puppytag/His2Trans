pub extern "C" fn HdfServiceObserverRecordRecycle(observerRecord: *mut crate::types::HdfServiceObserverRecord) {
    if !observerRecord.is_null() {
        unsafe {
            crate::compat::HdfSListFlush(&mut (*observerRecord).subscribers, crate::compat::HdfServiceSubscriberDelete);
            let _ = crate::compat::OsalMutexDestroy(&mut (*observerRecord).obsRecMutex);
            (*observerRecord).obsRecMutex.realMutex = std::ptr::null_mut();
            crate::compat::OsalMemFree(observerRecord as *mut ::core::ffi::c_void);
        }
    }
}