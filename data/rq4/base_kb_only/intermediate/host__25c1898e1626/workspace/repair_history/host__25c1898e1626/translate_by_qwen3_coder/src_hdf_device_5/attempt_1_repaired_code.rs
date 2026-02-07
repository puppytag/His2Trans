pub extern "C" fn HdfDeviceDetach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    use crate::types::*;
    
    if devInst.is_null() || devNode.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    // CONTAINER_OF: device = (struct HdfDevice *)((char *)(devInst) - (char *)&((struct HdfDevice *)0)->super)
    // Calculate offset using memoffset-style calculation without null deref
    let device: *mut HdfDevice = unsafe {
        let offset = core::mem::offset_of!(HdfDevice, super_);
        (devInst as *mut u8).sub(offset) as *mut HdfDevice
    };
    
    // Extract device IDs and compare: ((deviceId >> 8) & ((1 << 16) - 1))
    let device_id = unsafe { (*device).deviceId };
    let dev_node_id = unsafe { (*devNode).devId };
    
    let mask: u32 = (1 << 16) - 1;
    let device_id_masked = (device_id >> 8) & mask;
    let dev_node_id_masked = (dev_node_id >> 8) & mask;
    
    if device_id_masked != dev_node_id_masked {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: device detach unknown devnode \0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfDeviceDetach\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_DEV_ERR_NO_DEVICE;
    }
    
    // DListRemove inline implementation
    unsafe {
        if !(*devNode).entry.next.is_null() {
            let entry = &mut (*devNode).entry;
            (*entry.prev).next = entry.next;
            (*entry.next).prev = entry.prev;
            entry.prev = std::ptr::null_mut();
            entry.next = std::ptr::null_mut();
        }
    }
    
    // Call UnlaunchNode if present
    unsafe {
        if let Some(unlaunch_fn) = (*devNode).super_.UnlaunchNode {
            unlaunch_fn(devNode);
        }
    }
    
    HDF_SUCCESS
}