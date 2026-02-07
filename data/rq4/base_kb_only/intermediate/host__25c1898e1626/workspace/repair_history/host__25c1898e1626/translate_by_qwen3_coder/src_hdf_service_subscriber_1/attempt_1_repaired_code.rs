pub extern "C" fn HdfServiceSubscriberObtain(callback: crate::types::SubscriberCallback, devid: crate::types::devid_t) -> *mut crate::types::HdfServiceSubscriber {
    let serviceSubscriber = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::HdfServiceSubscriber>() as u32) as *mut crate::types::HdfServiceSubscriber
    };
    if !serviceSubscriber.is_null() {
        unsafe {
            (*serviceSubscriber).state = crate::types::HDF_SUBSCRIBER_STATE_PENDING;
            (*serviceSubscriber).devId = devid;
            (*serviceSubscriber).callback = callback;
        }
    }
    serviceSubscriber
}