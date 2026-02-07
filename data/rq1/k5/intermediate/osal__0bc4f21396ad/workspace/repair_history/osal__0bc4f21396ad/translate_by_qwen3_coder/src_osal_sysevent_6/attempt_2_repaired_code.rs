pub extern "C" fn HdfSysEventNotifyRegister(notifierNode: *mut crate::types::HdfSysEventNotifyNode, classSet: u64) -> i32 {
    if notifierNode.is_null() {
        return -1;
    }
    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();
    if notifier.is_null() {
        return -2;
    }
    unsafe {
        let mutex_ptr = &mut (*notifier).mutex as *mut crate::compat::OsalMutex;
        let _ = crate::compat::OsalMutexLock(mutex_ptr);
        let entry = &mut (*notifierNode).listNode;
        let head = &mut (*notifier).notifyNodeList;
        let head_prev = (*head).prev;
        let head_prev_mut = head_prev as *mut crate::types::DListHead;
        (*entry).next = head;
        (*entry).prev = head_prev;
        if !head_prev.is_null() {
            (*head_prev_mut).next = entry;
        }
        (*head).prev = entry;
        (*notifierNode).classFilter = classSet;
        let ret = crate::src_osal_sysevent::InitKeventIoServiceListenerLocked(notifier);
        if ret != 0 {
            let entry = &mut (*notifierNode).listNode;
            let entry_prev = (*entry).prev;
            let entry_next = (*entry).next;
            let entry_prev_mut = entry_prev as *mut crate::types::DListHead;
            let entry_next_mut = entry_next as *mut crate::types::DListHead;
            if !entry_prev.is_null() {
                (*entry_prev_mut).next = entry_next;
            }
            if !entry_next.is_null() {
                (*entry_next_mut).prev = entry_prev;
            }
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
        }
        let _ = crate::compat::OsalMutexUnlock(mutex_ptr);
        ret
    }
}