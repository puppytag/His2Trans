pub extern "C" fn HdfDeviceDetach(devInst: *mut crate::types::IHdfDevice, devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    use crate::types::*;
    if devInst.is_null() || devNode.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    let device = unsafe {
        let base = std::ptr::null::<HdfDevice>() as *const u8;
        let super_ptr = &(*base.cast::<HdfDevice>()).super_ as *const IHdfDevice as *const u8;
        let offset = base.offset_from(super_ptr);
        (devInst as *mut u8).offset(-offset) as *mut HdfDevice
    };
    let device_id_mask = unsafe { ((*device).deviceId >> 8) & ((1u32 << 16) - 1) };
    let devnode_id_mask = unsafe { ((*devNode).devId >> 8) & ((1u32 << 16) - 1) };
    if device_id_mask != devnode_id_mask {
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
            let prev = (*entry).prev;
            let next = (*entry).next;
            if !prev.is_null() {
                (*prev).next = next;
            }
            if !next.is_null() {
                (*next).prev = prev;
            }
            (*entry).prev = std::ptr::null_mut();
            (*entry).next = std::ptr::null_mut();
        }
        if let Some(f) = (*devNode).super_.UnlaunchNode {
            f(devNode);
        }
    }
    HDF_SUCCESS
}