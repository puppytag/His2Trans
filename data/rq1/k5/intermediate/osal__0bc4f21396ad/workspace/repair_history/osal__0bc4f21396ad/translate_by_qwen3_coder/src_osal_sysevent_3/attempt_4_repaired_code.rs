fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    use crate::src_osal_sysevent::FinishEvent;
    use libc::c_void;
    const HDF_ERR_INVALID_PARAM: i32 = -1;
    const HDF_ERR_INVALID_OBJECT: i32 = -2;
    const HDF_FAILURE: i32 = -3;
    const HDF_SUCCESS: i32 = 0;
    const HDF_SYSEVENT: u32 = 0xFADE;
    unsafe {
        let notifier = (*listener).priv as *mut c_void;
        if notifier.is_null() {
            return HDF_ERR_INVALID_PARAM;
        }
        if id != HDF_SYSEVENT {
            return HDF_ERR_INVALID_OBJECT;
        }
        let mut receivedEvent: *const c_void = std::ptr::null();
        let mut receivedEventLen: u32 = 0;
        if HdfSbufReadBuffer(data, &mut receivedEvent as *mut *const c_void, &mut receivedEventLen) == 0
            || receivedEventLen != std::mem::size_of::<c_void>() as u32
        {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"usysevent\0".as_ptr() as *const i8, b"failed to read kevent object\0".as_ptr() as *const i8);
            return HDF_FAILURE;
        }
        let mut eventContent = HdfSbufReadString(data);
        if eventContent.is_null() {
            eventContent = b"\0".as_ptr() as *const i8;
        }
        let notifier_mutex = notifier as *mut OsalMutex;
        let _ = OsalMutexLock(notifier_mutex);
        let notify_node_list = notifier as *mut c_void;
        let mut notifyNode = (*notify_node_list).offset(0) as *mut c_void;
        while !notifyNode.is_null() && notifyNode != notify_node_list {
            let class_filter = *(notifyNode as *const u64);
            let event_class = *(receivedEvent as *const u64);
            if (event_class & class_filter) != 0 {
                let callback = *(notifyNode.offset(8) as *const Option<unsafe extern "C" fn(*mut c_void, u64, u64, *const i8)>);
                if let Some(cb) = callback {
                    cb(notifyNode, event_class, *(receivedEvent.offset(8) as *const u64), eventContent);
                }
            }
            notifyNode = *(notifyNode as *const *mut c_void);
        }
        let sync_token = *(receivedEvent.offset(16) as *const u64);
        if sync_token != 0 {
            let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent as *const c_void);
        }
        let _ = OsalMutexUnlock(notifier_mutex);
        HDF_SUCCESS
    }
}