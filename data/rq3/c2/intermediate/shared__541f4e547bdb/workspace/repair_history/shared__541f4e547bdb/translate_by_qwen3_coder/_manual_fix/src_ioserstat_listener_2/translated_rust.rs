pub extern "C" fn IoServiceStatusListenerNewInstance() -> *mut crate::types::ServiceStatusListener {
    let listener = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::IoServiceStatusListener>() as u32)
    } as *mut crate::types::IoServiceStatusListener;
    
    if listener.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        (*listener).ioservListener.onReceive = Some(OnIoServiceEventReceive);
        (*listener).ioservListener.priv_ = listener as *mut ::core::ffi::c_void;
        
        &mut (*listener).svcstatListener as *mut crate::types::ServiceStatusListener
    }
}