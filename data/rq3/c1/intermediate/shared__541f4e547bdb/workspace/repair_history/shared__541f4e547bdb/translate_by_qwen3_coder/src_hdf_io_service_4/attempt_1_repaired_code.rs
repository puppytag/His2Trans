pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        #[linkage = "extern_weak"]
        static HdfIoServiceAdapterRemove: Option<unsafe extern "C" fn(*mut crate::types::HdfIoService)>;
    }
    
    unsafe {
        if let Some(func) = HdfIoServiceAdapterRemove {
            func(service);
        }
    }
}