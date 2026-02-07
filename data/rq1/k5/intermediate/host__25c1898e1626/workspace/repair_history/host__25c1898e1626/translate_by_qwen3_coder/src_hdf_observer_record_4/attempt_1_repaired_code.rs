pub extern "C" fn HdfServiceObserverRecordNotifySubscribers(record: *mut crate::types::HdfServiceObserverRecord, deviceId: crate::types::devid_t, policy: u16) {
    use crate::types::*;
    let mut it = HdfSListIterator {
        stepOnNext: 0,
        prev: std::ptr::null_mut(),
        curr: std::ptr::null_mut(),
    };
    if record.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, "observer_record".as_ptr() as *const i8, "%{public}s: record is null".as_ptr() as *const i8, "HdfServiceObserverRecordNotifySubscribers".as_ptr() as *const i8);
        return;
    }
    unsafe {
        OsalMutexLock(&mut (*record).obsRecMutex);
        HdfSListIteratorInit(&mut it as *mut HdfSListIterator, &(*record).subscribers as *const HdfSList);
        while HdfSListIteratorHasNext(&it as *const HdfSListIterator) {
            let subscriber = HdfSListIteratorNext(&mut it as *mut HdfSListIterator) as *mut HdfServiceSubscriber;
            if deviceId == (*subscriber).devId || policy != SERVICE_POLICY_PRIVATE as u16 {
                (*subscriber).state = HDF_SUBSCRIBER_STATE_READY as u32;
                if let Some(cb) = (*subscriber).callback.OnServiceConnected {
                    cb((*subscriber).callback.deviceObject, (*record).publisher);
                }
            }
        }
        OsalMutexUnlock(&mut (*record).obsRecMutex);
    }
}