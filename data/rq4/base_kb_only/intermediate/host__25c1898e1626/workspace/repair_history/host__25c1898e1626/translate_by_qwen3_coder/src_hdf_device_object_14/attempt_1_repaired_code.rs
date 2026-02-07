pub extern "C" fn HdfDeviceObjectRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let mut ret: ::core::ffi::c_int = HDF_FAILURE;
    
    // CONTAINER_OF: compute offset using memoffset-style calculation
    let offset = std::mem::offset_of!(HdfDeviceNode, deviceObject);
    let devNode: *mut HdfDeviceNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    let driverLoader: *mut IDriverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
    
    // Null checks
    if dev.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    unsafe {
        if (*devNode).driverName.is_null() || (*devNode).device.is_null() || driverLoader.is_null() {
            return HDF_ERR_INVALID_PARAM;
        }
        
        if (*driverLoader).GetDriver.is_none() {
            return HDF_ERR_INVALID_PARAM;
        }
        
        // devNode->driver = driverLoader->GetDriver(devNode->driverName)
        let get_driver_fn = (*driverLoader).GetDriver.unwrap();
        (*devNode).driver = get_driver_fn((*devNode).driverName);
        
        if (*devNode).driver.is_null() {
            return HDF_DEV_ERR_NO_DEVICE;
        }
        
        // ret = devNode->device->super.Attach(&devNode->device->super, devNode)
        let device_ptr = (*devNode).device;
        if let Some(attach_fn) = (*device_ptr).super_.Attach {
            ret = attach_fn(&mut (*device_ptr).super_ as *mut IHdfDevice, devNode);
        } else {
            return HDF_DEV_ERR_ATTACHDEV_FAIL;
        }
        
        if ret != HDF_SUCCESS {
            return HDF_DEV_ERR_ATTACHDEV_FAIL;
        }
    }
    
    ret
}