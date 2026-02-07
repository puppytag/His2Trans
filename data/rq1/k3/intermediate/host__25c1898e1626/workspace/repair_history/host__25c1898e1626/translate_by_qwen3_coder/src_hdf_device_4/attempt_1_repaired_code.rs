fn HdfDeviceAttach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    let mut ret: i32;
    let device = devInst as *mut crate::types::HdfDevice;
    let nodeIf = devNode as *mut crate::types::IDeviceNode;
    unsafe {
        if device.is_null() || nodeIf.is_null() || (*nodeIf).LaunchNode.is_none() {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const _,
                b"failed to attach device, input params invalid\0".as_ptr() as *const _,
            );
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
        if (*devNode).devId == 0
            && crate::src_hdf_device::AcquireNodeDeivceId(
                device,
                &mut (*devNode).devId as *mut crate::types::devid_t,
            ) != crate::types::HDF_SUCCESS
        {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const _,
                b"failed to attach device, invalid device id\0".as_ptr() as *const _,
            );
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
        if !(*devNode).token.is_null() {
            (*(*devNode).token).devid = (*devNode).devId;
        }
        if let Some(f) = (*nodeIf).LaunchNode {
            ret = f(devNode);
        } else {
            ret = crate::types::HDF_ERR_INVALID_PARAM;
        }
        if ret == crate::types::HDF_SUCCESS {
            let entry_ptr = &mut (*devNode).entry as *mut crate::types::DListHead;
            let head_ptr = &mut (*device).devNodes as *mut crate::types::DListHead;
            (*entry_ptr).next = head_ptr;
            (*entry_ptr).prev = (*head_ptr).prev;
            (*(*head_ptr).prev).next = entry_ptr;
            (*head_ptr).prev = entry_ptr;
            crate::src_hdf_device::UpdateDeivceNodeIdIndex(device, (*devNode).devId);
        }
        ret
    }
}