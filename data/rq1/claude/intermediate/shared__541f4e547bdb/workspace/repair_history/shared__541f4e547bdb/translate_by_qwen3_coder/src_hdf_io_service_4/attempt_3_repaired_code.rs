pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        static HdfIoServiceAdapterRemove: Option<unsafe extern "C" fn(service: *mut crate::types::HdfIoService)>;
    }
    
    unsafe {
        if let Some(f) = HdfIoServiceAdapterRemove {
            f(service);
        }
    }
}