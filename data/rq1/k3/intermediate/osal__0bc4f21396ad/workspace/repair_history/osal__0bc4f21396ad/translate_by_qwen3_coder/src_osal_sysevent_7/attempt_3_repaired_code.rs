pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut crate::types::HdfSysEventNotifyNode) {
    if notifierNode.is_null() {
        return;
    }
    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();
    if notifier.is_null() {
        return;
    }
    unsafe {
        let mutex_ptr = &mut (*notifier).mutex;
        let _ = crate::compat::OsalMutexLock(mutex_ptr);
        let list_node_ptr = &mut (*notifierNode).listNode;
        let prev = (*list_node_ptr).prev;
        let next = (*list_node_ptr).next;
        if !prev.is_null() {
            (*prev).next = next;
        }
        if !next.is_null() {
            (*next).prev = prev;
        }
        (*list_node_ptr).prev = std::ptr::null_mut();
        (*list_node_ptr).next = std::ptr::null_mut();
        let notify_node_list_ptr = &(*notifier).notifyNodeList;
        if (*notify_node_list_ptr).next == notify_node_list_ptr as *const _ {
            crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
        }
        let _ = crate::compat::OsalMutexUnlock(mutex_ptr);
    }
}