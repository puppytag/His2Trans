pub extern "C" fn HdfPmRegisterPowerListener(deviceObject: *mut crate::types::HdfDeviceObject, listener: *const crate::types::IPowerEventListener) -> ::core::ffi::c_int {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        let __mptr = deviceObject as *const crate::types::HdfDeviceObject;
        devNode = (__mptr as *const u8).offset(-((&(*(std::ptr::null::<crate::types::HdfDeviceNode>())).deviceObject) as *const _ as isize))) as *mut crate::types::HdfDeviceNode;
    }
    return crate::src_hdf_device_node::HdfDeviceNodeAddPowerStateListener(devNode, listener;
}