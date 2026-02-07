pub extern "C" fn IoServiceStatusListenerNewInstance() -> *mut crate::types::ServiceStatusListener {
    let listener = unsafe { libc::calloc(1, std::mem::size_of::<crate::types::IoServiceStatusListener>()) } as *mut crate::types::IoServiceStatusListener;
    if listener.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*listener).ioservListener.onReceive = Some(crate::src_ioserstat_listener::OnIoServiceEventReceive as crate::types::OnDevEventReceive);
        (*listener).ioservListener.priv_ = listener as *mut ::core::ffi::c_void;
        &mut (*listener).svcstatListener
    }
}