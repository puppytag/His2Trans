fn HdfDeviceAttach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    let device = devInst as *mut crate::types::HdfDevice;
    let node_if = devNode as *mut crate::types::IDeviceNode;
    if device.is_null() || node_if.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"hdf_device\0".as_ptr() as *const _,
            b"failed to attach device, input params invalid\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let launch_node = unsafe { (*node_if).LaunchNode };
    if launch_node.is_none() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            b"hdf_device\0".as_ptr() as *const _,
            b"failed to attach device, input params invalid\0".as_ptr() as *const _,
        ) };
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_id = unsafe { (*devNode).devId };
    if dev_id == 0 {
        let ret = unsafe { crate::src_hdf_device::AcquireNodeDeivceId(device, &mut (*devNode).devId as *mut crate::types::devid_t) };
        if ret != crate::types::HDF_SUCCESS {
            let _ = unsafe { crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const _,
                b"failed to attach device, invalid device id\0".as_ptr() as *const _,
            ) };
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
    }
    let token = unsafe { (*devNode).token };
    if !token.is_null() {
        unsafe { (*token).devid = (*devNode).devId };
    }
    let ret = unsafe { launch_node.unwrap()(devNode) };
    if ret == crate::types::HDF_SUCCESS {
        let entry = unsafe { &mut (*devNode).entry as *mut crate::types::DListHead };
        let head = unsafe { &mut (*device).devNodes as *mut crate::types::DListHead };
        unsafe {
            (*entry).next = head;
            (*entry).prev = (*head).prev;
            (*(*head).prev).next = entry;
            (*head).prev = entry;
        }
        unsafe { crate::src_hdf_device::UpdateDeivceNodeIdIndex(device, (*devNode).devId) };
    }
    ret
}