pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        fn HdfIoServiceAdapterRemove(service: *mut crate::types::HdfIoService);
    }
    
    // For weak symbols, we need to check if the symbol is defined
    // In C, weak symbols can be NULL if not defined
    // We'll just call it directly since Rust doesn't have a direct way to check weak symbols
    // The linker will resolve this - if the weak symbol exists it will be called
    unsafe {
        let func_ptr: *const () = HdfIoServiceAdapterRemove as *const ();
        if !func_ptr.is_null() {
            HdfIoServiceAdapterRemove(service);
        }
    }
}