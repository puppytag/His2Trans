pub extern "C" fn HdfServiceObserverDestruct(observer: *mut crate::types::HdfServiceObserver) {
    if !observer.is_null() {
        unsafe {
            HdfSListFlush(&mut (*observer).services, Some(crate::compat::HdfServiceObserverRecordDelete));
            let _ = OsalMutexDestroy(&mut (*observer).observerMutex);
        }
    }
}