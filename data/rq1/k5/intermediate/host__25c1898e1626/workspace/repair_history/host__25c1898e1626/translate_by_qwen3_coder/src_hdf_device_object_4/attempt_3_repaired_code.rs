pub extern "C" fn HdfPmUnregisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() {
        return;
    }
    unsafe {
        let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize;
        devNode = (deviceObject as *const u8).offset(-offset) as *mut crate::types::HdfDeviceNode;
    }
    crate::src_hdf_device_node::HdfDeviceNodeRemovePowerStateListener(devNode, listener);
}