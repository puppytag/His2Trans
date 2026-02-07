pub extern "C" fn HdfServiceObserverRemoveRecord(observer: *mut crate::types::HdfServiceObserver, svcName: *const ::core::ffi::c_char) {
    use crate::compat::*;
    use crate::types::*;
    
    let mut serviceRecord: *mut HdfServiceObserverRecord = std::ptr::null_mut();
    let serviceKey: u32 = unsafe { HdfStringMakeHashKey(svcName, 0) };
    
    if observer.is_null() || svcName.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE as u32,
                LOG_WARN,
                0xD002510,
                b"service_observer\0".as_ptr() as *const ::core::ffi::c_char,
                b"observer or svcName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    serviceRecord = unsafe {
        HdfSListSearch(
            &mut (*observer).services as *mut HdfSList,
            serviceKey,
            Some(crate::src_hdf_observer_record::HdfServiceObserverRecordCompare),
        ) as *mut HdfServiceObserverRecord
    };
    
    if !serviceRecord.is_null() {
        unsafe {
            OsalMutexLock(&mut (*observer).observerMutex as *mut OsalMutex);
            HdfSListRemove(
                &mut (*observer).services as *mut HdfSList,
                &mut (*serviceRecord).entry as *mut HdfSListNode,
            );
            OsalMutexUnlock(&mut (*observer).observerMutex as *mut OsalMutex);
            crate::src_hdf_observer_record::HdfServiceObserverRecordRecycle(serviceRecord);
        }
    }
}