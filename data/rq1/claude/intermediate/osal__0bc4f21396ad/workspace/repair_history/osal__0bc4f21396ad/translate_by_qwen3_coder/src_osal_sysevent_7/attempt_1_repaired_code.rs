pub extern "C" fn HdfSysEventNotifyUnregister(notifierNode: *mut HdfSysEventNotifyNode) {
    if notifierNode.is_null() {
        return;
    }

    let notifier = crate::src_osal_sysevent::HdfSysEventNotifierGetInstance();

    if notifier.is_null() {
        return;
    }

    unsafe {
        OsalMutexLock(&mut (*notifier).mutex as *mut OsalMutex);

        // DListRemove inline implementation
        // DListHead is opaque, so we use pointer arithmetic
        let entry = std::ptr::addr_of_mut!((*notifierNode).listNode) as *mut *mut ::core::ffi::c_void;
        let next_ptr = entry; // next is first field
        let prev_ptr = entry.add(1); // prev is second field
        
        let next = *next_ptr;
        let prev = *prev_ptr;
        
        // prev->next = next
        let prev_next = prev as *mut *mut ::core::ffi::c_void;
        *prev_next = next;
        
        // next->prev = prev
        let next_prev = (next as *mut *mut ::core::ffi::c_void).add(1);
        *next_prev = prev;
        
        // entry->prev = NULL, entry->next = NULL
        *prev_ptr = std::ptr::null_mut();
        *next_ptr = std::ptr::null_mut();

        // DListIsEmpty inline implementation
        let head = std::ptr::addr_of!((*notifier).notifyNodeList) as *const *const ::core::ffi::c_void;
        let head_next = *head;
        let is_empty = head_next as usize == head as usize;
        
        if is_empty {
            crate::src_osal_sysevent::DeInitKeventIoServiceListenerLocked(notifier);
        }

        OsalMutexUnlock(&mut (*notifier).mutex as *mut OsalMutex);
    }
}