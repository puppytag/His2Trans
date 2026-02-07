fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    unsafe {
        let notifier = (*listener).priv_ as *mut crate::types::HdfSysEventNotifier;
        if notifier.is_null() {
            return crate::types::HDF_ERR_INVALID_PARAM as i32;
        }

        if id != 0xFADE {
            return crate::types::HDF_ERR_INVALID_OBJECT!() as i32;
        }

        let mut receivedEvent: *const ::core::ffi::c_void = std::ptr::null();
        let mut receivedEventLen: u32 = 0;

        if crate::compat::HdfSbufReadBuffer(data, &mut receivedEvent, &mut receivedEventLen) == false
            || receivedEventLen as usize != std::mem::size_of::<crate::types::HdfSysEvent>()
        {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE!(),
                crate::types::LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to read kevent object\0".as_ptr() as *const i8,
            );
            return crate::types::HDF_FAILURE as i32;
        }

        let receivedEvent = receivedEvent as *const crate::types::HdfSysEvent;

        let eventContent = crate::compat::HdfSbufReadString(data);
        let eventContent = if eventContent.is_null() {
            b"\0".as_ptr() as *const i8
        } else {
            eventContent
        };

        crate::compat::OsalMutexLock(&mut (*notifier).mutex as *mut crate::types::OsalMutex);

        let head = &mut (*notifier).notifyNodeList as *mut crate::types::DListHead;
        let mut cur = (*head).next;
        while cur != head {
            let offset = std::mem::offset_of!(crate::types::HdfSysEventNotifyNode, listNode);
            let notifyNode = (cur as *mut u8).sub(offset) as *mut crate::types::HdfSysEventNotifyNode;
            
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

        crate::compat::OsalMutexUnlock(&mut (*notifier).mutex as *mut crate::types::OsalMutex);

        crate::types::HDF_SUCCESS as i32
    }
}