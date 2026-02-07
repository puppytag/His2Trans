pub extern "C" fn HdfPmReleaseDeviceAsync(deviceObject: *mut crate::types::HdfDeviceObject) {
    if deviceObject.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: input param is invalid\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmReleaseDeviceAsync\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }

    // container_of macro: get HdfDeviceNode from embedded deviceObject field
    // Use offset_of pattern without null pointer dereference
    let offset = core::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    let devNode: *mut crate::types::HdfDeviceNode = 
        ((deviceObject as *mut u8).wrapping_sub(offset)) as *mut crate::types::HdfDeviceNode;

    unsafe {
        crate::src_hdf_power_manager::HdfPmTaskPut(
            (*devNode).powerToken,
            crate::types::HDF_PM_REQUEST_RELEASE,
        );
    }
}