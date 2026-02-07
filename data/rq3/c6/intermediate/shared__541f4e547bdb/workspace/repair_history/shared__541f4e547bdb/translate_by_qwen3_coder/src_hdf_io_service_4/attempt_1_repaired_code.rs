pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    unsafe {
        if crate::compat::(HdfIoServiceAdapterRemove.is_some)() {
            (crate::compat::HdfIoServiceAdapterRemove.unwrap())(service);
        }
    }
}