pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        fn HdfIoServiceAdapterRemove(service: *mut crate::types::HdfIoService);
    }
    
    // For weak symbols in stable Rust, we need to use a different approach.
    // Since the C code checks if the weak symbol is non-null, we can use
    // a raw pointer check. The weak symbol address can be obtained and checked.
    unsafe {
        let func_ptr: *const () = HdfIoServiceAdapterRemove as *const ();
        if !func_ptr.is_null() {
            HdfIoServiceAdapterRemove(service);
        }
    }
}