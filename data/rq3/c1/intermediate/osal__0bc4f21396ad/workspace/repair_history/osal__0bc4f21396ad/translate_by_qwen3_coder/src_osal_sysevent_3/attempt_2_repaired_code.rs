fn OnKEventReceived(listener: *mut crate::types::HdfDevEventlistener, service: *mut crate::types::HdfIoService, id: u32, data: *mut crate::types::HdfSBuf) -> i32 {
    use crate::compat::*;
    
    const HDF_ERR_INVALID_PARAM: i32 = 0x80000003u32 as i32;
    const HDF_ERR_INVALID_OBJECT: i32 = 0x80000004u32 as i32;
    const HDF_FAILURE: i32 = 0x80000001u32 as i32;
    const HDF_SUCCESS: i32 = 0;
    const HDF_SYSEVENT: u32 = 0xFADE;
    const LOG_CORE: crate::compat::LogType = 3;
    const LOG_ERROR: crate::compat::LogLevel = 6;
    
    let notifier: *mut crate::types::HdfSysEventNotifier = unsafe { (*listener).priv_ as *mut crate::types::HdfSysEventNotifier };
    if notifier.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    if id != HDF_SYSEVENT {
        return HDF_ERR_INVALID_OBJECT;
    }
    
    let mut receivedEvent: *mut crate::types::HdfSysEvent = std::ptr::null_mut();
    let mut receivedEventLen: u32 = 0;
    
    let read_ok = unsafe {
        HdfSbufReadBuffer(
            data,
            &mut receivedEvent as *mut *mut crate::types::HdfSysEvent as *mut *const ::core::ffi::c_void,
            &mut receivedEventLen
        )
    };
    
    if !read_ok || receivedEventLen != std::mem::size_of::<crate::types::HdfSysEvent>() as u32 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to read kevent object\0".as_ptr() as *const i8
            );
        }
        return HDF_FAILURE;
    }
    
    let mut eventContent: *const i8 = unsafe { HdfSbufReadString(data) };
    if eventContent.is_null() {
        eventContent = b"\0".as_ptr() as *const i8;
    }
    
    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        OsalMutexLock(mutex_ptr);
    }
    
    // DListHead is opaque - we need to use raw pointer arithmetic
    // The list structure: each node has a DListHead at offset listNode
    // We iterate by treating the memory as raw pointers
    unsafe {
        let list_head_ptr = std::ptr::addr_of!((*notifier).notifyNodeList) as *mut ::core::ffi::c_void;
        // First pointer in DListHead is 'next'
        let mut current = *(list_head_ptr as *mut *mut ::core::ffi::c_void);
        
        while current != list_head_ptr {
            let offset = std::mem::offset_of!(crate::types::HdfSysEventNotifyNode, listNode);
            let notifyNode = (current as *mut u8).wrapping_sub(offset) as *mut crate::types::HdfSysEventNotifyNode;
            
            if ((*receivedEvent).eventClass & (*notifyNode).classFilter) != 0 {
                if let Some(callback) = (*notifyNode).callback {
                    let _ = callback(
                        notifyNode,
                        (*receivedEvent).eventClass,
                        (*receivedEvent).eventid,
                        eventContent
                    );
                }
            }
            // Move to next: first field of current DListHead is 'next'
            current = *(current as *mut *mut ::core::ffi::c_void);
        }
        
        if (*receivedEvent).syncToken != 0 {
            let _ = crate::src_osal_sysevent::FinishEvent(service, receivedEvent as *const crate::types::HdfSysEvent);
        }
        
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        OsalMutexUnlock(mutex_ptr);
    }
    
    HDF_SUCCESS
}