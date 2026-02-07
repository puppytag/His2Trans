pub extern "C" fn HdfDeviceObjectUnRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    if dev.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let dev_node = unsafe {
        (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize))
            as *mut crate::types::HdfDeviceNode
    };
    if dev_node.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let device = unsafe { (*dev_node).device };
    if device.is_null() {
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let detach = unsafe { (*device).super_.Detach };
    if let Some(f) = detach {
        unsafe { f(device as *mut crate::types::IHdfDevice, dev_node) }
    } else {
        crate::types::HDF_ERR_INVALID_OBJECT
    }
}