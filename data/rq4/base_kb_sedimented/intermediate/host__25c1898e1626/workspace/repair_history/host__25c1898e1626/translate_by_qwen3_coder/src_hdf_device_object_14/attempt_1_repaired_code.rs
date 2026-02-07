pub extern "C" fn HdfDeviceObjectRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let mut ret: ::core::ffi::c_int = HDF_FAILURE;
    
    // Calculate offset of deviceObject field in HdfDeviceNode using a dummy aligned allocation
    // to avoid null pointer dereference at compile time
    let offset = core::mem::offset_of!(HdfDeviceNode, deviceObject);
    
    let devNode: *mut HdfDeviceNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    let driverLoader: *mut IDriverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
    
    // Null checks
    if dev.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to add device, param invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_ERR_INVALID_PARAM;
    }
    
    unsafe {
        if (*devNode).driverName.is_null() || (*devNode).device.is_null() || driverLoader.is_null() {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to add device, param invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return HDF_ERR_INVALID_PARAM;
        }
        
        if (*driverLoader).GetDriver.is_none() {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to add device, param invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return HDF_ERR_INVALID_PARAM;
        }
        
        // Get driver
        let get_driver_fn = (*driverLoader).GetDriver.unwrap();
        (*devNode).driver = get_driver_fn((*devNode).driverName);
        
        if (*devNode).driver.is_null() {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"can not found driver %{public}s\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return HDF_DEV_ERR_NO_DEVICE;
        }
        
        // Attach device
        let device_ptr = (*devNode).device;
        let super_ptr = &mut (*device_ptr).super_ as *mut IHdfDevice;
        
        if let Some(attach_fn) = (*super_ptr).Attach {
            ret = attach_fn(super_ptr, devNode);
        }
        
        if ret != HDF_SUCCESS {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to attach device %{public}s\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return HDF_DEV_ERR_ATTACHDEV_FAIL;
        }
    }
    
    ret
}