pub extern "C" fn HdfPmRegisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> ::core::ffi::c_int {
    if deviceObject.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    
    // Calculate offset of deviceObject field within HdfDeviceNode using offset_of
    // This avoids dereferencing a null pointer
    let offset = core::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    
    let devNode = unsafe {
        (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    };
    
    crate::src_hdf_device_node::HdfDeviceNodeAddPowerStateListener(devNode, listener)
}