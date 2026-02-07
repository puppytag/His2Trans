pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut HdfSysEventNotifyNode) {
    if notifierNode.is_null() {
        return;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return;
    }

    unsafe {
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        OsalMutexLock(mutex_ptr);

        // DListRemove inline implementation
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode);
        let prev = (*entry).prev;
        let next = (*entry).next;
        (*prev).next = next;
        (*next).prev = prev;
        (*entry).prev = std::ptr::null_mut();
        (*entry).next = std::ptr::null_mut();

        // DListIsEmpty check
        let head = std::ptr::addr_of!((*notifier).notifyNodeList);
        let is_empty = (*head).next == head as *mut _;
        if is_empty {
            crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
        }

        OsalMutexUnlock(mutex_ptr);
    }
}