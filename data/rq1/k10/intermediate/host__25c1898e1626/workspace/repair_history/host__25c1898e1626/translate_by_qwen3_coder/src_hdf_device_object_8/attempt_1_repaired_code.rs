pub extern "C" fn HdfPmReleaseDeviceAsync(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const _,
                b"%{public}s: input param is invalid\0".as_ptr() as *const _,
                b"HdfPmReleaseDeviceAsync\0".as_ptr() as *const _,
            );
        }
        return;
    }
    unsafe {
        devNode = (deviceObject as *mut u8).offset(
            -(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)
        ) as *mut crate::types::HdfDeviceNode;
        if !(*devNode).powerToken.is_null() {
            crate::src_hdf_power_manager::HdfPmTaskPut(
                (*devNode).powerToken,
                crate::types::HDF_PM_REQUEST_RELEASE,
            );
        }
    }
}