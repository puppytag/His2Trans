pub extern "C" fn DevHostServiceDelDevice(inst: *mut crate::types::IDevHostService, devId: crate::types::devid_t) -> ::core::ffi::c_int {
    let hostService = inst as *mut crate::types::DevHostService;
    let deviceId = ((devId >> 8) & ((1 << 16) - 1)) as u16;
    let device = crate::src_devhost_service::DevHostServiceFindDevice(hostService, deviceId);
    if device.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_WARN,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"failed to del device, device is not exist\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_SUCCESS;
    }
    let devNode = unsafe {
        if let Some(f) = (*device).super_.GetDeviceNode {
            f(&mut (*device).super_ as *mut crate::types::IHdfDevice, devId)
        } else {
            std::ptr::null_mut()
        }
    };
    if devNode.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"failed to del device, not exist\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }
    if unsafe { (*device).super_.Detach.is_none() } {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"failed to del device, invalid device\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_ERR_INVALID_OBJECT;
    }
    let detach_result = unsafe {
        if let Some(f) = (*device).super_.Detach {
            f(&mut (*device).super_ as *mut crate::types::IHdfDevice, devNode)
        } else {
            crate::types::HDF_FAILURE
        }
    };
    if detach_result != crate::types::HDF_SUCCESS {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"failed to detach device\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
    let is_empty = unsafe { ((*device).devNodes.next == &mut (*device).devNodes as *mut crate::types::DListHead) };
    if is_empty {
        crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_DEBUG,
            0xD002510,
            b"devhost_service\0".as_ptr() as *const _,
            b"%{public}s add device success\0".as_ptr() as *const _,
        );
    }
    crate::types::HDF_SUCCESS
}