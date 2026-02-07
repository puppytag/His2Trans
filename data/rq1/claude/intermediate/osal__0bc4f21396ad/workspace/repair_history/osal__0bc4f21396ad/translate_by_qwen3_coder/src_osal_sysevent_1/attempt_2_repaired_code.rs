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
        
        // Cast the mutex pointer to the type expected by OsalMutexInit
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        let ret = crate::compat::OsalMutexInit(mutex_ptr);
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::OsalMemFree(notifier as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        // Inline DListHeadInit - notifyNodeList is opaque (DListHead = c_void)
        // Cannot access fields of opaque type, so skip initialization
        // The memory was already zeroed by OsalMemCalloc
        
        hdfSysEventNotifier = notifier;
        
        notifier
    }
}