fn AcquireNodeDeivceId(device: *mut crate::types::HdfDevice, devid: *mut crate::types::devid_t) -> i32 {
    let mut node_id: crate::types::devid_t = 0;
    let usable_id: crate::types::devid_t;
    unsafe {
        if (*device).devidIndex >= ((1 << 8) - 1) as u16 {
            return crate::types::HDF_FAILURE;
        }
        if (*device).devidIndex < 129 {
            (*device).devidIndex = 129;
            node_id = (*device).devidIndex as crate::types::devid_t;
        } else {
            usable_id = crate::src_hdf_device::FindUsableDevNodeId(device);
            if usable_id <= (*device).devidIndex as crate::types::devid_t {
                node_id = usable_id;
            } else {
                (*device).devidIndex += 1;
                node_id = (*device).devidIndex as crate::types::devid_t;
            }
        }
        if devid.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"hdf_device\0".as_ptr() as *const _,
                b"params invalid *devid\0".as_ptr() as *const _,
            );
            return crate::types::HDF_ERR_INVALID_PARAM;
        }
        let device_id = (*device).deviceId;
        let high_part = ((device_id >> (16 + 8)) as u32) << (16 + 8);
        let mid_part = (((device_id >> 8) & ((1 << 16) - 1)) as u32) << 8;
        *devid = high_part | mid_part | node_id;
    }
    crate::types::HDF_SUCCESS
}