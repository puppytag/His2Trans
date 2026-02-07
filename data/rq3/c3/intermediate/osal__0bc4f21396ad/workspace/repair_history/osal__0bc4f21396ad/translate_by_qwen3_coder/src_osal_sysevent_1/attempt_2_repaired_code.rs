fn HdfSysEventNotifierGetInstance() -> *mut crate::types::HdfSysEventNotifier {
    static mut hdfSysEventNotifier: *mut crate::types::HdfSysEventNotifier = std::ptr::null_mut();
    
    unsafe {
        if !hdfSysEventNotifier.is_null() {
            return hdfSysEventNotifier;
        }
        
        let notifier = crate::compat::OsalMemCalloc(
            std::mem::size_of::<crate::types::HdfSysEventNotifier>() as u32
        ) as *mut crate::types::HdfSysEventNotifier;
        
        if notifier.is_null() {
            return std::ptr::null_mut();
        }
        
        let ret = crate::compat::OsalMutexInit(std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex);
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::OsalMemFree(notifier as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        // DListHeadInit inline - use raw pointer without type annotation
        let list_ptr = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        (*notifier).notifyNodeList.next = list_ptr;
        (*notifier).notifyNodeList.prev = list_ptr;
        
        hdfSysEventNotifier = notifier;
        
        notifier
    }
}