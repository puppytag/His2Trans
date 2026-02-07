pub extern "C" fn IoServiceStatusListenerFree(listener: *mut crate::types::ServiceStatusListener) {
    if listener.is_null() {
        return;
    }
    
    // CONTAINER_OF macro: get pointer to IoServiceStatusListener from its svcstatListener field
    // offset = &((struct IoServiceStatusListener *)0)->svcstatListener
    let offset = unsafe {
        &(*(std::ptr::null::<crate::types::IoServiceStatusListener>())).svcstatListener as *const _ as usize
    };
    
    let ioserv_listener = unsafe {
        (listener as *mut u8).sub(offset) as *mut crate::types::IoServiceStatusListener
    };
    
    unsafe {
        crate::compat::OsalMemFree(ioserv_listener as *mut ::core::ffi::c_void);
    }
}