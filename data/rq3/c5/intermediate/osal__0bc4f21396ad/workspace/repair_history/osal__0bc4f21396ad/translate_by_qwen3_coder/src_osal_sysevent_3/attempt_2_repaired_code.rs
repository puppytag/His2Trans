fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    unsafe {
        let notifier = (*listener).priv_ as *mut crate::types::HdfSysEventNotifier;
        if notifier.is_null() {
            return crate::types::HDF_ERR_INVALID_PARAM as i32;
        }

        if id != 0xFADE {
            return crate::types::HDF_ERR_INVALID_OBJECT!() as i32;
        }

        let mut receivedEvent: *const crate::types::HdfSysEvent = std::ptr::null();
        let mut receivedEventLen: u32 = 0;

        let read_ok = HdfSbufReadBuffer(
            data,
            &mut receivedEvent as *mut *const crate::types::HdfSysEvent as *mut *const ::core::ffi::c_void,
            &mut receivedEventLen,
        );

        if read_ok == 0 || receivedEventLen as usize != std::mem::size_of::<crate::types::HdfSysEvent>() {
            let _ = HiLogPrint(
                crate::types::LOG_CORE!(),
                crate::types::LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to read kevent object\0".as_ptr() as *const i8,
            );
            return crate::types::HDF_FAILURE as i32;
        }

        let mut eventContent = HdfSbufReadString(data);
        if eventContent.is_null() {
            eventContent = b"\0".as_ptr() as *const i8;
        }

        OsalMutexLock(&mut (*notifier).mutex);

        // DLIST_FOR_EACH_ENTRY iteration
        let head = &(*notifier).notifyNodeList as *const crate::types::DListHead;
        let listnode_offset = std::mem::offset_of!(crate::types::HdfSysEventNotifyNode, listNode);
        
        let mut cur = (*head).next;
        while cur != head as *mut crate::types::DListHead {
            let notifyNode = (cur as *mut u8).sub(listnode_offset) as *mut crate::types::HdfSysEventNotifyNode;
            
            if ((*receivedEvent).eventClass & (*notifyNode).classFilter) != 0 {
                if let Some(cb) = (*notifyNode).callback {
                    let _ = cb(
                        notifyNode,
                        (*receivedEvent).eventClass,
                        (*receivedEvent).eventid,
                        eventContent,
                    );
                }
            }
            
            cur = (*cur).next;
        }

        if (*receivedEvent).syncToken != 0 {
            let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent);
        }

        OsalMutexUnlock(&mut (*notifier).mutex);

        crate::types::HDF_SUCCESS as i32
    }
}