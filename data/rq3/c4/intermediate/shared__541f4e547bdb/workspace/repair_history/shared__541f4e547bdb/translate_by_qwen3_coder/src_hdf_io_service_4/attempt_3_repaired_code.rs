pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        fn HdfIoServiceAdapterRemove(service: *mut crate::types::HdfIoService);
    }
    
    let fn_ptr: Option<unsafe extern "C" fn(*mut crate::types::HdfIoService)> = Some(HdfIoServiceAdapterRemove);
    
    if let Some(f) = fn_ptr {
        unsafe { f(service) };
    }
}