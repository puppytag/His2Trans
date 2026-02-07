pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return -3i32;
    }

    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        OsalMutexLock(mutex_ptr);
        
        // DListInsertTail inline implementation
        let entry = &mut (*notifierNode).listNode;
        let head = &mut (*notifier).notifyNodeList;
        (*entry).next = head as *mut _;
        (*entry).prev = (*head).prev;
        (*(*head).prev).next = entry as *mut _;
        (*head).prev = entry as *mut _;
        
        (*notifierNode).classFilter = classSet;
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        if ret != crate::types::HDF_SUCCESS {
            // DListRemove inline implementation
            let entry = &mut (*notifierNode).listNode;
            (*(*entry).prev).next = (*entry).next;
            (*(*entry).next).prev = (*entry).prev;
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
        }
        OsalMutexUnlock(mutex_ptr);

        ret
    }
}