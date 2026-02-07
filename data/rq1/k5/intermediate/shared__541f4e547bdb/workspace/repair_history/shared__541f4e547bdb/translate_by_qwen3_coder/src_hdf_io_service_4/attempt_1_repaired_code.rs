pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    if !(HdfIoServiceAdapterRemove.is_null)() {
        unsafe {
            HdfIoServiceAdapterRemove(service);
        }
    }
}