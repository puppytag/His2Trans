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
        
        // DListInsertTail inline implementation
        let entry_ptr = std::ptr::addr_of_mut!((*notifierNode).listNode);
        let head_ptr = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        (*entry_ptr).next = head_ptr;
        (*entry_ptr).prev = (*head_ptr).prev;
        (*(*head_ptr).prev).next = entry_ptr;
        (*head_ptr).prev = entry_ptr;
        
        (*notifierNode).classFilter = classSet;
        
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            // DListRemove inline implementation
            let entry_ptr2 = std::ptr::addr_of_mut!((*notifierNode).listNode);
            (*(*entry_ptr2).prev).next = (*entry_ptr2).next;
            (*(*entry_ptr2).next).prev = (*entry_ptr2).prev;
            (*entry_ptr2).prev = std::ptr::null_mut();
            (*entry_ptr2).next = std::ptr::null_mut();
        }
        
        OsalMutexUnlock(mutex_ptr);
        
        ret
    }
}