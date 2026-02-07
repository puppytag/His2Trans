fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    const HDF_ERR_INVALID_OBJECT: i32 = -4;
    const HDF_FAILURE: i32 = -1;
    
    unsafe {
        let notifier = (*listener).priv_ as *mut crate::types::HdfSysEventNotifier;
        if notifier.is_null() {
            return crate::types::HDF_ERR_INVALID_PARAM as i32;
        }

        if id != 0xFADE {
            return HDF_ERR_INVALID_OBJECT;
        }

        let mut receivedEvent: *const ::core::ffi::c_void = std::ptr::null();
        let mut receivedEventLen: u32 = 0;

        if crate::compat::HdfSbufReadBuffer(data, &mut receivedEvent, &mut receivedEventLen) == false
            || receivedEventLen as usize != std::mem::size_of::<crate::types::HdfSysEvent>()
        {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE!(),
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to read kevent object\0".as_ptr() as *const i8,
            );
            return HDF_FAILURE;
        }

        let receivedEvent = receivedEvent as *const crate::types::HdfSysEvent;

        let eventContent = crate::compat::HdfSbufReadString(data);
        let eventContent = if eventContent.is_null() {
            b"\0".as_ptr() as *const i8
        } else {
            eventContent
        };

        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        crate::compat::OsalMutexLock(mutex_ptr);

        let head = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        let mut cur = (*head).next;
        while cur != head as *mut crate::types::DListHead {
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

        crate::compat::OsalMutexUnlock(mutex_ptr);

        HDF_SUCCESS
    }
}