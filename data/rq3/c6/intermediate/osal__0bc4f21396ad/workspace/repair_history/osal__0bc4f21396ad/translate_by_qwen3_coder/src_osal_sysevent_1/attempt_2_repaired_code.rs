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
        
        let mutex_ptr = std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex;
        let ret = crate::compat::OsalMutexInit(mutex_ptr);
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::OsalMemFree(notifier as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        // DListHeadInit inline implementation
        // DListHead is opaque, so we use pointer arithmetic
        let head_ptr = std::ptr::addr_of_mut!((*notifier).notifyNodeList) as *mut *mut ::core::ffi::c_void;
        let head_as_void = std::ptr::addr_of_mut!((*notifier).notifyNodeList) as *mut ::core::ffi::c_void;
        // next is at offset 0, prev is at offset sizeof(pointer)
        *head_ptr = head_as_void;
        *(head_ptr.add(1)) = head_as_void;
        
        hdfSysEventNotifier = notifier;
        
        notifier
    }
}