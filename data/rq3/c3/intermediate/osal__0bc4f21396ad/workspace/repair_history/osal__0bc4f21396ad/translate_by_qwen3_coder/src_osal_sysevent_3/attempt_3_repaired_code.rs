fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    use crate::compat::*;
    use crate::types::*;
    
    let notifier: *mut HdfSysEventNotifier = unsafe { (*listener).priv_ as *mut HdfSysEventNotifier };
    if notifier.is_null() {
        return HDF_ERR_INVALID_PARAM as i32;
    }
    
    if id != 0xFADE {
        return HDF_ERR_INVALID_OBJECT!() as i32;
    }
    
    let mut receivedEvent: *const HdfSysEvent = std::ptr::null();
    let mut receivedEventLen: u32 = 0;
    
    let read_ok = unsafe {
        HdfSbufReadBuffer(
            data,
            &mut receivedEvent as *mut *const HdfSysEvent as *mut *const ::core::ffi::c_void,
            &mut receivedEventLen
        )
    };
    
    if !read_ok || receivedEventLen != std::mem::size_of::<HdfSysEvent>() as u32 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to read kevent object\0".as_ptr() as *const i8
            );
        }
        return HDF_FAILURE as i32;
    }
    
    let mut eventContent = unsafe { HdfSbufReadString(data) };
    if eventContent.is_null() {
        eventContent = b"\0".as_ptr() as *const i8;
    }
    
    unsafe {
        OsalMutexLock(&mut (*notifier).mutex);
    }
    
    // Iterate through the notify node list
    unsafe {
        let list_head = &(*notifier).notifyNodeList;
        let offset = &(*(std::ptr::null::<HdfSysEventNotifyNode>())).listNode as *const _ as usize;
        
        let mut current = (*list_head).next;
        while current != list_head as *const DListHead as *mut DListHead {
            let notifyNode = (current as *mut u8).sub(offset) as *mut HdfSysEventNotifyNode;
            
            if ((*receivedEvent).eventClass & (*notifyNode).classFilter) != 0 {
                if let Some(cb) = (*notifyNode).callback {
                    let _ = cb(
                        notifyNode,
                        (*receivedEvent).eventClass,
                        (*receivedEvent).eventid,
                        eventContent
                    );
                }
            }
            
            current = (*current).next;
        }
        
        if (*receivedEvent).syncToken != 0 {
            let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent);
        }
        
        OsalMutexUnlock(&mut (*notifier).mutex);
    }
    
    HDF_SUCCESS as i32
}