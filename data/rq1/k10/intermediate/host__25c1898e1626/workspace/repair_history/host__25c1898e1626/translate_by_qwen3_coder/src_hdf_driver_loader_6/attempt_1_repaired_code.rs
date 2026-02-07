pub extern "C" fn HdfDriverLoaderGetInstance() -> *mut crate::types::IDriverLoader {
    static mut INSTANCE: *mut crate::types::IDriverLoader = std::ptr::null_mut();
    unsafe {
        if INSTANCE.is_null() {
            INSTANCE = crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DRIVER_LOADER as i32) as *mut crate::types::IDriverLoader;
        }
        INSTANCE
    }
}