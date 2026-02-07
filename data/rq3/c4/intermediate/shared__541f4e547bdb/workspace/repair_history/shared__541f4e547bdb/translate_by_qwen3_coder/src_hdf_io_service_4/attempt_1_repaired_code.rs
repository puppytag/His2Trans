pub extern "C" fn HdfIoServiceRemove(service: *mut crate::types::HdfIoService) {
    unsafe {
        if (HdfIoServiceAdapterRemove.is_some)() {
            (HdfIoServiceAdapterRemove.unwrap())(service);
        }
    }
}