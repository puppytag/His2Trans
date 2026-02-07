pub extern "C" fn IoServiceStatusListenerFree(listener: *mut crate::types::ServiceStatusListener) {
    if listener.is_null() {
        return;
    }
    // CONTAINER_OF macro: (char*)(listener) - (char*)&((struct IoServiceStatusListener*)0)->svcstatListener
    // This computes the offset of svcstatListener within IoServiceStatusListener and subtracts it from listener
    let offset = unsafe {
        &(*(std::ptr::null::<crate::types::IoServiceStatusListener>())).svcstatListener as *const _ as usize
    };
    let ioserv_listener = (listener as *mut u8).wrapping_sub(offset) as *mut crate::types::IoServiceStatusListener;
    unsafe {
        crate::compat::OsalMemFree(ioserv_listener as *mut ::core::ffi::c_void);
    }
}