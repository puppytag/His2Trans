fn HdfDeviceDetachWithDevid(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t) -> i32 {
    let _dev = unsafe { (device as *mut u8).sub(std::mem::offset_of!(crate::types::HdfDevice, super_)) };
    let devNode = crate::src_hdf_device::HdfDeviceGetDeviceNode(device, devid);
    if devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"hdf_device\0".as_ptr() as *const _, b"devNode is NULL\0".as_ptr() as *const _) };
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }
    crate::src_hdf_device::HdfDeviceDetach(device, devNode)
}