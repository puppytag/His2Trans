pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return -6i32;
    }

    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut ::core::ffi::c_void as *mut crate::compat::OsalMutex;
        OsalMutexLock(mutex_ptr);
        
        // DListInsertTail inline: insert notifierNode->listNode at tail of notifier->notifyNodeList
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode) as *mut u8;
        let head = std::ptr::addr_of_mut!((*notifier).notifyNodeList) as *mut u8;
        
        // DListHead layout: next, prev (both pointers)
        let ptr_size = std::mem::size_of::<*mut u8>();
        
        let entry_next_ptr = entry as *mut *mut u8;
        let entry_prev_ptr = entry.add(ptr_size) as *mut *mut u8;
        let head_prev_ptr = head.add(ptr_size) as *mut *mut u8;
        
        let head_prev = *head_prev_ptr;
        let head_prev_next_ptr = head_prev as *mut *mut u8;
        
        *entry_next_ptr = head;
        *entry_prev_ptr = head_prev;
        *head_prev_next_ptr = entry;
        *head_prev_ptr = entry;
        
        (*notifierNode).classFilter = classSet;
        
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            // DListRemove inline: remove notifierNode->listNode from list
            let entry = std::ptr::addr_of_mut!((*notifierNode).listNode) as *mut u8;
            let entry_next_ptr = entry as *mut *mut u8;
            let entry_prev_ptr = entry.add(ptr_size) as *mut *mut u8;
            
            let entry_prev = *entry_prev_ptr;
            let entry_next = *entry_next_ptr;
            
            let entry_prev_next_ptr = entry_prev as *mut *mut u8;
            let entry_next_prev_ptr = (entry_next).add(ptr_size) as *mut *mut u8;
            
            *entry_prev_next_ptr = entry_next;
            *entry_next_prev_ptr = entry_prev;
            *entry_prev_ptr = std::ptr::null_mut();
            *entry_next_ptr = std::ptr::null_mut();
        }
        
        OsalMutexUnlock(mutex_ptr);
        
        ret
    }
}