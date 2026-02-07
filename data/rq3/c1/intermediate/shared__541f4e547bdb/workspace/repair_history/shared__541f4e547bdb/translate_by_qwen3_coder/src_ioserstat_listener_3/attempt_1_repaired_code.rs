pub extern "C" fn IoServiceStatusListenerFree(listener: *mut crate::types::ServiceStatusListener) {
    if listener.is_null() {
        return;
    }
    
    // CONTAINER_OF macro: get pointer to IoServiceStatusListener from its svcstatListener field
    // Use offset_of! equivalent via MaybeUninit to avoid null pointer dereference
    let offset = {
        let dummy = std::mem::MaybeUninit::<crate::types::IoServiceStatusListener>::uninit();
        let base_ptr = dummy.as_ptr();
        unsafe {
            let field_ptr = std::ptr::addr_of!((*base_ptr).svcstatListener);
            (field_ptr as *const u8).offset_from(base_ptr as *const u8) as usize
        }
    };
    
    let ioserv_listener = unsafe {
        (listener as *mut u8).sub(offset) as *mut crate::types::IoServiceStatusListener
    };
    
    unsafe {
        crate::compat::OsalMemFree(ioserv_listener as *mut ::core::ffi::c_void);
    }
}