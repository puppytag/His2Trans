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
        
        let ret = crate::compat::OsalMutexInit(std::ptr::addr_of_mut!((*notifier).mutex));
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::OsalMemFree(notifier as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        // Inline DListHeadInit - use raw pointer arithmetic to avoid type mismatch
        let list_ptr = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        (*list_ptr).next = list_ptr;
        (*list_ptr).prev = list_ptr;
        
        hdfSysEventNotifier = notifier;
        
        notifier
    }
}