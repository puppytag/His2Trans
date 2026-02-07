pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    if let Some(f) = HdfIoServiceAdapterRemove {
        unsafe {
            f(service);
        }
    }
}