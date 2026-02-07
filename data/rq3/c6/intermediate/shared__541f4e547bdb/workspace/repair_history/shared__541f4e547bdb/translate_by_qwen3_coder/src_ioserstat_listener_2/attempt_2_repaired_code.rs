pub extern "C" fn IoServiceStatusListenerNewInstance() -> *mut crate::types::ServiceStatusListener {
    let listener: *mut crate::types::IoServiceStatusListener = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::IoServiceStatusListener>() as u32) as *mut crate::types::IoServiceStatusListener
    };
    
    if listener.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe extern "C" fn on_io_service_event_receive_wrapper(
        listener: *mut crate::types::HdfDevEventlistener,
        service: *mut crate::types::HdfIoService,
        id: u32,
        data: *mut crate::types::HdfSBuf,
    ) -> i32 {
        OnIoServiceEventReceive(listener, service, id, data)
    }
    
    unsafe {
        (*listener).ioservListener.onReceive = Some(on_io_service_event_receive_wrapper);
        (*listener).ioservListener.priv_ = listener as *mut ::core::ffi::c_void;
        
        &mut (*listener).svcstatListener as *mut crate::types::ServiceStatusListener
    }
}