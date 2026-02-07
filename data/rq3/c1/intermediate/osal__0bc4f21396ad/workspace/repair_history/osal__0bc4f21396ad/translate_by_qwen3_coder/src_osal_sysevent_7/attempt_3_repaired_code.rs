pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut HdfSysEventNotifyNode) {
    if notifierNode.is_null() {
        return;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return;
    }

    unsafe {
        OsalMutexLock(std::ptr::addr_of_mut!((*notifier).mutex) as *mut _);

        // DListRemove inline implementation
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode);
        (*(*entry).prev).next = (*entry).next;
        (*(*entry).next).prev = (*entry).prev;
        (*entry).prev = std::ptr::null_mut();
        (*entry).next = std::ptr::null_mut();

        // DListIsEmpty inline implementation
        let head = std::ptr::addr_of!((*notifier).notifyNodeList);
        let is_empty = (*head).next == head as *mut _;
        if is_empty {
            crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
        }

        OsalMutexUnlock(std::ptr::addr_of_mut!((*notifier).mutex) as *mut _);
    }
}