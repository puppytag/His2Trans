pub extern "C" fn HdfDeviceTokenNewInstance() -> *mut crate::types::IHdfDeviceToken {
    unsafe {
        crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVICE_TOKEN as i32) as *mut crate::types::IHdfDeviceToken
    }
}