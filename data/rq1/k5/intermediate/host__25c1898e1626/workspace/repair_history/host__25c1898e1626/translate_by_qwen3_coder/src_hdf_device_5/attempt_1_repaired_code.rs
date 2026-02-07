pub extern "C" fn HdfDeviceDetach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    use crate::types::*;
    if devInst.is_null() || devNode.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    let device = unsafe {
        let base = std::ptr::null::<HdfDevice>() as *const u8;
        let super_ptr = &(*(std::ptr::null::<HdfDevice>())).super_ as *const IHdfDevice as *const u8;
        let offset = (super_ptr as isize) - (base as isize);
        (devInst as *mut u8).offset(-offset) as *mut HdfDevice
    };
    let device_id_mask = unsafe { ((*device).deviceId >> 8) & 0xFFFF };
    let dev_node_id_mask = unsafe { ((*devNode).devId >> 8) & 0xFFFF };
    if device_id_mask != dev_node_id_mask {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const _,
                b"%{public}s: device detach unknown devnode \0".as_ptr() as *const _,
                b"HdfDeviceDetach\0".as_ptr() as *const _,
            )
        };
        return HDF_DEV_ERR_NO_DEVICE;
    }
    unsafe {
        if !(*devNode).entry.next.is_null() {
            let entry = &mut (*devNode).entry;
            if !entry.prev.is_null() {
                (*entry.prev).next = entry.next;
            }
            if !entry.next.is_null() {
                (*entry.next).prev = entry.prev;
            }
            entry.prev = std::ptr::null_mut();
            entry.next = std::ptr::null_mut();
        }
        if let Some(f) = (*devNode).super_.UnlaunchNode {
            f(devNode);
        }
    }
    HDF_SUCCESS
}