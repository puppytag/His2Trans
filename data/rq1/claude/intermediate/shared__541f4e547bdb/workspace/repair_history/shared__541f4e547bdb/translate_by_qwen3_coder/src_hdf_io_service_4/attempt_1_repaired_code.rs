pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        fn HdfIoServiceAdapterRemove(service: *mut crate::types::HdfIoService);
        static HdfIoServiceAdapterRemove: Option<unsafe extern "C" fn(service: *mut crate::types::HdfIoService)>;
    }
    
    unsafe {
        if (HdfIoServiceAdapterRemove.is_some)() {
            (HdfIoServiceAdapterRemove.unwrap())(service);
        }
    }
}