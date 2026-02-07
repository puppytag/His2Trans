fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    const HDF_SYSEVENT: u32 = 0xFADE;
    const HDF_ERR_INVALID_PARAM: i32 = -1;
    const HDF_ERR_INVALID_OBJECT: i32 = -2;
    const HDF_FAILURE: i32 = -1;
    const HDF_SUCCESS: i32 = 0;
    let notifier: *mut crate::types::HdfSysEventNotifier = unsafe {
        if listener.is_null() {
            return HDF_ERR_INVALID_PARAM;
        }
        (*listener).priv as *mut crate::types::HdfSysEventNotifier
    };
    if notifier.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    if id != HDF_SYSEVENT {
        return HDF_ERR_INVALID_OBJECT;
    }
    let mut receivedEvent: *const crate::types::HdfSysEvent = std::ptr::null();
    let mut receivedEventLen: u32 = 0;
    let read_ok = unsafe {
        HdfSbufReadBuffer(
            data,
            &mut (receivedEvent as *const *const crate::types::HdfSysEvent as *const *const ::core::ffi::c_void),
            &mut receivedEventLen,
        )
    };
    if !read_ok || receivedEventLen != std::mem::size_of::<crate::types::HdfSysEvent>() as u32 {
        let _ = unsafe { HiLogPrint(LOG_CORE!(), LOG_ERROR, 0xD002510, b"usysevent\0".as_ptr() as *const i8, b"failed to read kevent object\0".as_ptr() as *const i8) };
        return HDF_FAILURE;
    }
    let mut eventContent: *const i8 = unsafe { HdfSbufReadString(data) };
    if eventContent.is_null() {
        eventContent = b"\0".as_ptr() as *const i8;
    }
    unsafe {
        OsalMutexLock(&mut (*notifier).mutex);
    }
    let notify_node_list: *mut crate::types::DListHead = unsafe { &mut (*notifier).notifyNodeList };
    let mut notifyNode: *mut crate::types::HdfSysEventNotifyNode = std::ptr::null_mut();
    unsafe {
        if !(*notify_node_list).next.is_null() {
            notifyNode = ((*notify_node_list).next as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfSysEventNotifyNode, listNode) as isize)) as *mut crate::types::HdfSysEventNotifyNode;
        }
    }
    while !notifyNode.is_null() && unsafe { &(*notifyNode).listNode } != notify_node_list {
        let class_filter = unsafe { (*notifyNode).classFilter };
        let event_class = unsafe { (*receivedEvent).eventClass };
        if (event_class & class_filter) != 0 {
            let callback = unsafe { (*notifyNode).callback };
            if let Some(cb) = callback {
                let _ = unsafe { cb(notifyNode, event_class, (*receivedEvent).eventid, eventContent) };
            }
        }
        let next_node: *mut crate::types::DListHead = unsafe { (*notifyNode).listNode.next };
        if next_node.is_null() || next_node == notify_node_list {
            break;
        }
        notifyNode = (next_node as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfSysEventNotifyNode, listNode) as isize)) as *mut crate::types::HdfSysEventNotifyNode;
    }
    let sync_token = unsafe { (*receivedEvent).syncToken };
    if sync_token != 0 {
        let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent);
    }
    unsafe {
        OsalMutexUnlock(&mut (*notifier).mutex);
    }
    HDF_SUCCESS
}