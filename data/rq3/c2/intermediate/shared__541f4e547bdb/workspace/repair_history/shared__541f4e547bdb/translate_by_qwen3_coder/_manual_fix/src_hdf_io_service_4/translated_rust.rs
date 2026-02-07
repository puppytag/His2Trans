pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        fn HdfIoServiceAdapterRemove(service: *mut crate::types::HdfIoService);
        static HdfIoServiceAdapterRemove: Option<unsafe extern "C" fn(*mut crate::types::HdfIoService)>;
    }
    
    unsafe {
        // Check if the weak symbol is non-null
        let func_ptr: *const () = HdfIoServiceAdapterRemove as *const ();
        if !func_ptr.is_null() {
            HdfIoServiceAdapterRemove(service);
        }
    }
}