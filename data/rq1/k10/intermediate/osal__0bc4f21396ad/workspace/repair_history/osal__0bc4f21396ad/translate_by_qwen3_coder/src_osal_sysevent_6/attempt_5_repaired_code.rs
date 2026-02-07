pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut crate::types::HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();
    if notifier.is_null() {
        return -1;
    }
    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        let _ = crate::compat::OsalMutexLock(mutex_ptr);
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode) as *mut crate::types::DListHead;
        let head = std::ptr::addr_of_mut!((*notifier).notifyNodeList) as *mut crate::types::DListHead;
        (*entry).next = head;
        (*entry).prev = (*head).prev;
        (*(*head).prev).next = entry;
        (*head).prev = entry;
        (*notifierNode).classFilter = classSet;
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        if ret != crate::types::HDF_SUCCESS {
            let entry = std::ptr::addr_of_mut!((*notifierNode).listNode) as *mut crate::types::DListHead;
            (*(*entry).prev).next = (*entry).next;
            (*(*entry).next).prev = (*entry).prev;
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
        }
        let _ = crate::compat::OsalMutexUnlock(mutex_ptr);
        ret
    }
}