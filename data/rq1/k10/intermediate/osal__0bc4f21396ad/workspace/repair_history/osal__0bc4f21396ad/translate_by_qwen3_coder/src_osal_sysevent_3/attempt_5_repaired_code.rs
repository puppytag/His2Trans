fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    use crate::types::*;
    use crate::compat::*;
    use crate::globals::*;
    use ::core::ffi::c_void;
    use ::libc;
    const HDF_SYSEVENT: u32 = 0xFADE;
    const HDF_ERR_INVALID_PARAM: i32 = -1;
    const HDF_ERR_INVALID_OBJECT: i32 = -2;
    const HDF_FAILURE: i32 = -3;
    const HDF_SUCCESS: i32 = 0;
    let notifier = unsafe { (*listener).priv as *mut HdfSysEventNotifier };
    if notifier.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    if id != HDF_SYSEVENT {
        return HDF_ERR_INVALID_OBJECT;
    }
    let mut received_event_ptr: *const c_void = std::ptr::null();
    let mut received_event_len: u32 = 0;
    let read_ok = unsafe {
        HdfSbufReadBuffer(
            data,
            &mut received_event_ptr as *mut *const c_void as *mut *const c_void,
            &mut received_event_len,
        )
    };
    if read_ok == 0 || received_event_len != std::mem::size_of::<HdfSysEvent>() as u32 {
        let _ = unsafe { HiLogPrint(LOG_CORE!(), LOG_ERROR, 0xD002510, b"usysevent\0".as_ptr() as *const i8, b"failed to read kevent object\0".as_ptr() as *const i8) };
        return HDF_FAILURE;
    }
    let received_event = received_event_ptr as *const HdfSysEvent;
    let event_content_ptr = unsafe { HdfSbufReadString(data) };
    let event_content = if event_content_ptr.is_null() {
        b"\0".as_ptr() as *const i8
    } else {
        event_content_ptr
    };
    unsafe { OsalMutexLock(&mut (*notifier).mutex) };
    let notify_node_list = unsafe { &(*notifier).notifyNodeList };
    let mut notify_node = unsafe {
        (notify_node_list.next as *mut u8).offset(-(std::mem::size_of::<HdfSysEventNotifyNode>() as isize)) as *mut HdfSysEventNotifyNode
    };
    while unsafe { &(*notify_node).listNode } != notify_node_list {
        let next = unsafe { (*notify_node).listNode.next };
        if unsafe { (*received_event).eventClass & (*notify_node).classFilter } != 0 {
            let cb = unsafe { (*notify_node).callback };
            if let Some(f) = cb {
                unsafe { f(notify_node, (*received_event).eventClass, (*received_event).eventid, event_content) };
            }
        }
        notify_node = unsafe {
            (next as *mut u8).offset(-(std::mem::size_of::<HdfSysEventNotifyNode>() as isize)) as *mut HdfSysEventNotifyNode
        };
    }
    if unsafe { (*received_event).syncToken } != 0 {
        let _ = crate::src_osal_sysevent::FinishEvent(service, received_event);
    }
    unsafe { OsalMutexUnlock(&mut (*notifier).mutex) };
    HDF_SUCCESS
}