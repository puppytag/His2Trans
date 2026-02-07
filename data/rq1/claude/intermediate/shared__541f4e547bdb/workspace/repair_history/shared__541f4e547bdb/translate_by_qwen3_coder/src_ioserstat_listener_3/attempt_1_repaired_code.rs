pub extern "C" fn IoServiceStatusListenerFree(listener: *mut crate::types::ServiceStatusListener) {
    if listener.is_null() {
        return;
    }
    
    // CONTAINER_OF macro: get pointer to IoServiceStatusListener from its svcstatListener field
    // Use offset_of! equivalent via MaybeUninit to avoid null pointer dereference
    let offset = {
        let uninit = std::mem::MaybeUninit::<crate::types::IoServiceStatusListener>::uninit();
        let base_ptr = uninit.as_ptr();
        unsafe {
            let field_ptr = std::ptr::addr_of!((*base_ptr).svcstatListener);
            (field_ptr as *const u8).offset_from(base_ptr as *const u8) as usize
        }
    };
    
    let ioservListener = unsafe {
        (listener as *mut u8).sub(offset) as *mut crate::types::IoServiceStatusListener
    };
    
    unsafe {
        crate::compat::OsalMemFree(ioservListener as *mut ::core::ffi::c_void);
    }
}