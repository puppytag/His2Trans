pub extern "C" fn HdfServiceObserverDestruct(observer: *mut crate::types::HdfServiceObserver) {
    if !observer.is_null() {
        unsafe {
            crate::compat::HdfSListFlush(
                &mut (*observer).services as *mut crate::types::HdfSList,
                Some(crate::src_hdf_service_observer::HdfServiceObserverRecordDelete),
            );
            crate::compat::OsalMutexDestroy(
                &mut (*observer).observerMutex as *mut crate::types::OsalMutex,
            );
        }
    }
}