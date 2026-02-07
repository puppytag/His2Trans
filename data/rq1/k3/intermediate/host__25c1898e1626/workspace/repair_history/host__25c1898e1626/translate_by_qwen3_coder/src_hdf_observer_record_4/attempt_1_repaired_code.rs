pub extern "C" fn HdfServiceObserverRecordNotifySubscribers(record: *mut crate::types::HdfServiceObserverRecord, deviceId: crate::types::devid_t, policy: u16) {
    let mut it = crate::types::HdfSListIterator {
        stepOnNext: 0,
        prev: std::ptr::null_mut(),
        curr: std::ptr::null_mut(),
    };
    if record.is_null() {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD002510,
            b"observer_record\0".as_ptr() as *const _,
            b"%{public}s: record is null\0".as_ptr() as *const _,
            b"HdfServiceObserverRecordNotifySubscribers\0".as_ptr() as *const _,
        );
        return;
    }
    unsafe {
        crate::compat::OsalMutexLock(&mut (*record).obsRecMutex);
        crate::compat::HdfSListIteratorInit(&mut it, &(*record).subscribers);
        while crate::compat::HdfSListIteratorHasNext(&it) {
            let subscriber = crate::compat::HdfSListIteratorNext(&mut it) as *mut crate::types::HdfServiceSubscriber;
            if deviceId == (*subscriber).devId || policy != crate::types::SERVICE_POLICY_PRIVATE as u16 {
                (*subscriber).state = crate::types::HDF_SUBSCRIBER_STATE_READY as u32;
                if let Some(cb) = (*subscriber).callback.OnServiceConnected {
                    cb((*subscriber).callback.deviceObject, (*record).publisher);
                }
            }
        }
        crate::compat::OsalMutexUnlock(&mut (*record).obsRecMutex);
    }
}