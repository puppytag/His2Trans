pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return -6i32;
    }

    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        OsalMutexLock(mutex_ptr);
        
        // DListInsertTail inline: insert notifierNode->listNode at tail of notifier->notifyNodeList
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode);
        let head = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        
        // Access DListHead fields via pointer arithmetic since type may be opaque
        // DListHead layout: next, prev (both *mut DListHead)
        let entry_next_ptr = entry as *mut *mut crate::types::DListHead;
        let entry_prev_ptr = (entry as *mut u8).add(std::mem::size_of::<*mut crate::types::DListHead>()) as *mut *mut crate::types::DListHead;
        let head_next_ptr = head as *mut *mut crate::types::DListHead;
        let head_prev_ptr = (head as *mut u8).add(std::mem::size_of::<*mut crate::types::DListHead>()) as *mut *mut crate::types::DListHead;
        
        let head_prev = *head_prev_ptr;
        let head_prev_next_ptr = head_prev as *mut *mut crate::types::DListHead;
        
        *entry_next_ptr = head;
        *entry_prev_ptr = head_prev;
        *head_prev_next_ptr = entry;
        *head_prev_ptr = entry;
        
        (*notifierNode).classFilter = classSet;
        
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            // DListRemove inline: remove notifierNode->listNode from list
            let entry = std::ptr::addr_of_mut!((*notifierNode).listNode);
            let entry_next_ptr = entry as *mut *mut crate::types::DListHead;
            let entry_prev_ptr = (entry as *mut u8).add(std::mem::size_of::<*mut crate::types::DListHead>()) as *mut *mut crate::types::DListHead;
            
            let entry_prev = *entry_prev_ptr;
            let entry_next = *entry_next_ptr;
            
            let entry_prev_next_ptr = entry_prev as *mut *mut crate::types::DListHead;
            let entry_next_prev_ptr = (entry_next as *mut u8).add(std::mem::size_of::<*mut crate::types::DListHead>()) as *mut *mut crate::types::DListHead;
            
            *entry_prev_next_ptr = entry_next;
            *entry_next_prev_ptr = entry_prev;
            *entry_prev_ptr = std::ptr::null_mut();
            *entry_next_ptr = std::ptr::null_mut();
        }
        
        OsalMutexUnlock(mutex_ptr);
        
        ret
    }
}