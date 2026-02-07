pub extern "C" fn DevHostServiceFreeInstance(service: *mut crate::types::IDevHostService) {
    if !service.is_null() {
        unsafe {
            crate::compat::HdfObjectManagerFreeObject(&mut (*service).object);
        }
    }
}