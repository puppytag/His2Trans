pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return crate::types::HDF_DEV_ERR_NO_MEMORY!() as i32;
    }

    unsafe {
        OsalMutexLock(&mut (*notifier).mutex);
        
        // DListInsertTail inline implementation
        let entry = &mut (*notifierNode).listNode;
        let head = &mut (*notifier).notifyNodeList;
        (*entry).next = head as *mut crate::types::DListHead;
        (*entry).prev = (*head).prev;
        (*(*head).prev).next = entry as *mut crate::types::DListHead;
        (*head).prev = entry as *mut crate::types::DListHead;
        
        (*notifierNode).classFilter = classSet;
        
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            // DListRemove inline implementation
            let entry = &mut (*notifierNode).listNode;
            (*(*entry).prev).next = (*entry).next;
            (*(*entry).next).prev = (*entry).prev;
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
        }
        
        OsalMutexUnlock(&mut (*notifier).mutex);
        
        ret
    }
}