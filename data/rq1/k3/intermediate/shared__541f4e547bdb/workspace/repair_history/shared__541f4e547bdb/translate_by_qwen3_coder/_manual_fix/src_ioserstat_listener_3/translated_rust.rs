pub extern "C" fn IoServiceStatusListenerFree(listener: *mut crate::types::ServiceStatusListener) {
    if listener.is_null() {
        return;
    }
    let offset = unsafe {
        &(*(0 as *const crate::types::IoServiceStatusListener)).svcstatListener as *const _ as usize
    };
    let ioservListener = (listener as *mut u8).wrapping_sub(offset) as *mut crate::types::IoServiceStatusListener;
    unsafe {
        crate::compat::OsalMemFree(ioservListener as *mut ::core::ffi::c_void);
    }
}