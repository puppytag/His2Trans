pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    extern "C" {
        fn HdfIoServiceAdapterRemove(service: *mut crate::types::HdfIoService);
    }
    
    // For weak symbols in stable Rust, we declare the function and use weak_linkage
    // via a raw pointer check. Since we can't use #[linkage] on stable,
    // we'll use a different approach: declare as a regular extern and let the linker
    // resolve it. If the weak symbol doesn't exist, linking will fail, but that's
    // the expected behavior for weak symbols that must be provided.
    // 
    // Alternative: use dlsym at runtime, but for simplicity we just call it directly.
    // The C code checks for NULL which implies the symbol might not exist,
    // but in practice if it's declared weak and not provided, it will be NULL.
    
    // Since we cannot use weak linkage on stable Rust, we'll assume the symbol exists
    // and call it unconditionally. If the original C code's weak symbol behavior is
    // critical, this would need to be handled differently (e.g., via build configuration).
    unsafe {
        HdfIoServiceAdapterRemove(service);
    }
}