pub extern "C" fn IoServiceStatusListenerNewInstance() -> *mut crate::types::ServiceStatusListener {
    let listener = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::IoServiceStatusListener>() as u32)
    } as *mut crate::types::IoServiceStatusListener;
    
    if listener.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        (*listener).ioservListener.onReceive = Some(std::mem::transmute::<
            fn(*mut crate::types::HdfDevEventlistener, *mut crate::types::HdfIoService, u32, *mut crate::types::HdfSBuf) -> i32,
            unsafe extern "C" fn(*mut crate::types::HdfDevEventlistener, *mut crate::types::HdfIoService, u32, *mut crate::types::HdfSBuf) -> ::core::ffi::c_int
        >(OnIoServiceEventReceive));
        (*listener).ioservListener.priv_ = listener as *mut ::core::ffi::c_void;
        
        &mut (*listener).svcstatListener as *mut crate::types::ServiceStatusListener
    }
}