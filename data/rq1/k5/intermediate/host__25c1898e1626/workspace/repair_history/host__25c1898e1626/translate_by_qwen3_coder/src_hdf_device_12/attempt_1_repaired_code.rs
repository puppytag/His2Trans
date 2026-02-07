pub extern "C" fn HdfDeviceNewInstance() -> *mut crate::types::HdfDevice {
    unsafe {
        crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVICE as i32) as *mut crate::types::HdfDevice
    }
}