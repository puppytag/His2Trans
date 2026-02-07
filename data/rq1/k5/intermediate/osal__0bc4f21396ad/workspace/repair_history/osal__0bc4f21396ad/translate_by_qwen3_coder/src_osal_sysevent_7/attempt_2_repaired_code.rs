pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut crate::types::HdfSysEventNotifyNode) {
    if notifierNode.is_null() {
        return;
    }
    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();
    if notifier.is_null() {
        return;
    }
    unsafe {
        let _ = crate::compat::OsalMutexLock(std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex);
        let list_node_ptr = &mut (*notifierNode).listNode;
        (*list_node_ptr).prev.as_mut().unwrap().next = (*list_node_ptr).next;
        (*list_node_ptr).next.as_mut().unwrap().prev = (*list_node_ptr).prev;
        (*list_node_ptr).prev = std::ptr::null_mut();
        (*list_node_ptr).next = std::ptr::null_mut();
        if (*notifier).notifyNodeList.next == &mut (*notifier).notifyNodeList {
            crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
        }
        let _ = crate::compat::OsalMutexUnlock(std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex);
    }
}