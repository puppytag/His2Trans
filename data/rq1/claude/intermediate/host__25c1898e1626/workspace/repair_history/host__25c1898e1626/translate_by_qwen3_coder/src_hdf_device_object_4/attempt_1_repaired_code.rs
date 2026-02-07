pub extern "C" fn HdfPmUnregisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) {
    if deviceObject.is_null() {
        return;
    }
    
    // Calculate offset of deviceObject field within HdfDeviceNode using offset_of
    let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    
    let devNode = unsafe {
        (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    };
    
    crate::src_hdf_device_node::HdfDeviceNodeRemovePowerStateListener(devNode, listener);
}