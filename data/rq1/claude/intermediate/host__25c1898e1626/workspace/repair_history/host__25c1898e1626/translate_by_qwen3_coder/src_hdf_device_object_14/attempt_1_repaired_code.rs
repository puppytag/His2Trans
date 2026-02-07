pub extern "C" fn HdfDeviceObjectRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let mut ret: ::core::ffi::c_int = HDF_FAILURE;
    
    // CONTAINER_OF: compute offset using memoffset-style calculation
    let offset = std::mem::offset_of!(HdfDeviceNode, deviceObject);
    let devNode: *mut HdfDeviceNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    let driverLoader: *mut IDriverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
    
    if dev.is_null() {
        unsafe {
            crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, param invalid\0".as_ptr() as *const ::core::ffi::c_char);
        }
        return HDF_ERR_INVALID_PARAM;
    }
    
    let driver_name = unsafe { (*devNode).driverName };
    let device = unsafe { (*devNode).device };
    
    if driver_name.is_null() || device.is_null() || driverLoader.is_null() {
        unsafe {
            crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, param invalid\0".as_ptr() as *const ::core::ffi::c_char);
        }
        return HDF_ERR_INVALID_PARAM;
    }
    
    let get_driver_fn = unsafe { (*driverLoader).GetDriver };
    if get_driver_fn.is_none() {
        unsafe {
            crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const ::core::ffi::c_char, b"failed to add device, param invalid\0".as_ptr() as *const ::core::ffi::c_char);
        }
        return HDF_ERR_INVALID_PARAM;
    }
    
    let driver = unsafe { get_driver_fn.unwrap()(driver_name) };
    unsafe { (*devNode).driver = driver; }
    
    if driver.is_null() {
        unsafe {
            crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const ::core::ffi::c_char, b"can not found driver %{public}s\0".as_ptr() as *const ::core::ffi::c_char, driver_name);
        }
        return HDF_DEV_ERR_NO_DEVICE;
    }
    
    let attach_fn = unsafe { (*device).super_.Attach };
    if let Some(attach) = attach_fn {
        ret = unsafe { attach(&mut (*device).super_ as *mut IHdfDevice, devNode) };
    }
    
    if ret != HDF_SUCCESS {
        unsafe {
            crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const ::core::ffi::c_char, b"failed to attach device %{public}s\0".as_ptr() as *const ::core::ffi::c_char, driver_name);
        }
        return HDF_DEV_ERR_ATTACHDEV_FAIL;
    }
    
    ret
}