pub extern "C" fn HdfServiceObserverRecordNotifySubscribers(record: *mut crate::types::HdfServiceObserverRecord, deviceId: crate::types::devid_t, policy: u16) {
    use crate::compat::*;
    use crate::types::*;
    
    if record.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD002510,
                b"observer_record\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: record is null\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfServiceObserverRecordNotifySubscribers\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    let mut it: HdfSListIterator = unsafe { std::mem::zeroed() };
    
    unsafe {
        OsalMutexLock(&mut (*record).obsRecMutex);
        HdfSListIteratorInit(&mut it, &(*record).subscribers as *const HdfSList);
        
        while HdfSListIteratorHasNext(&it as *const HdfSListIterator) {
            let subscriber = HdfSListIteratorNext(&mut it) as *mut HdfServiceSubscriber;
            if !subscriber.is_null() {
                if deviceId == (*subscriber).devId || policy != SERVICE_POLICY_PRIVATE as u16 {
                    (*subscriber).state = HDF_SUBSCRIBER_STATE_READY;
                    if let Some(on_connected) = (*subscriber).callback.OnServiceConnected {
                        on_connected((*subscriber).callback.deviceObject, (*record).publisher);
                    }
                }
            }
        }
        
        OsalMutexUnlock(&mut (*record).obsRecMutex);
    }
}