pub extern "C" fn HdfPmUnregisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() {
        return;
    }
    unsafe {
        let __mptr = deviceObject as *const crate::types::HdfDeviceObject;
        devNode = (__mptr as *const u8).offset(-((&(*(std::ptr::null::<crate::types::HdfDeviceNode>())).deviceObject) as *const _ as isize))) as *mut crate::types::HdfDeviceNode;
    }
    crate::src_hdf_device_node::HdfDeviceNodeRemovePowerStateListener(devNode, listener;
}