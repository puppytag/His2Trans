fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    use crate::compat::*;
    use crate::globals::*;
    use crate::types::*;
    const HDF_SYSEVENT: u32 = 0xFADE;
    const HDF_ERR_INVALID_PARAM: i32 = -1;
    const HDF_ERR_INVALID_OBJECT: i32 = -2;
    const HDF_FAILURE: i32 = -1;
    const HDF_SUCCESS: i32 = 0;
    unsafe {
        let notifier = (*listener).priv as *mut HdfSysEventNotifier;
        if notifier.is_null() {
            return HDF_ERR_INVALID_PARAM;
        }
        if id != HDF_SYSEVENT {
            return HDF_ERR_INVALID_OBJECT;
        }
        let mut receivedEvent: *const HdfSysEvent = std::ptr::null();
        let mut receivedEventLen: u32 = 0;
        if HdfSbufReadBuffer(data, &mut (receivedEvent as *const _ as *const *const ::core::ffi::c_void), &mut receivedEventLen) == 0
            || receivedEventLen != std::mem::size_of::<HdfSysEvent>() as u32
        {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"usysevent\0".as_ptr() as *const i8, b"failed to read kevent object\0".as_ptr() as *const i8);
            return HDF_FAILURE;
        }
        let mut eventContent = HdfSbufReadString(data);
        if eventContent.is_null() {
            eventContent = b"\0".as_ptr() as *const i8;
        }
        let _ = OsalMutexLock(&mut (*notifier).mutex);
        let mut notifyNode: *mut HdfSysEventNotifyNode = std::ptr::null_mut();
        let list_head = &mut (*notifier).notifyNodeList;
        let mut pos = (*list_head).next;
        while pos != list_head as *mut _ {
            notifyNode = (pos as *mut u8).offset(-(std::mem::offset_of!(HdfSysEventNotifyNode, listNode) as isize)) as *mut HdfSysEventNotifyNode;
            if (*receivedEvent).eventClass & (*notifyNode).classFilter != 0 {
                let cb = (*notifyNode).callback;
                if let Some(f) = cb {
                    let _ = f(notifyNode, (*receivedEvent).eventClass, (*receivedEvent).eventid, eventContent);
                }
            }
            pos = (*pos).next;
        }
        if (*receivedEvent).syncToken != 0 {
            let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent);
        }
        let _ = OsalMutexUnlock(&mut (*notifier).mutex);
        HDF_SUCCESS
    }
}