pub extern "C" fn HdfDeviceTokenFreeInstance(token: *mut crate::types::IHdfDeviceToken) {
    if !token.is_null() {
        unsafe {
            crate::compat::HdfObjectManagerFreeObject(&mut (*token).object as *mut crate::types::HdfObject);
        }
    }
}