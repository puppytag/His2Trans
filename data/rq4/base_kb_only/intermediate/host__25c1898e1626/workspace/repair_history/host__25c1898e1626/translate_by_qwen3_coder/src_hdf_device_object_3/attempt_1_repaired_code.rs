pub extern "C" fn HdfPmRegisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> ::core::ffi::c_int {
    if deviceObject.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    
    // HDF_SLIST_CONTAINER_OF macro expansion:
    // Get the HdfDeviceNode from the embedded deviceObject field
    // devNode = (struct HdfDeviceNode *)((char *)deviceObject - offsetof(struct HdfDeviceNode, deviceObject))
    let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    let devNode = unsafe {
        (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    };
    
    crate::src_hdf_device_node::HdfDeviceNodeAddPowerStateListener(devNode, listener)
}