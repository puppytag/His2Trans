pub extern "C" fn DeviceDriverBind(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    let mut driverEntry: *const crate::types::HdfDriverEntry = std::ptr::null();
    if devNode.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        driverEntry = (*devNode).driver.as_ref().unwrap().entry;
        if (*devNode).policy == crate::types::SERVICE_POLICY_PUBLIC as u16
            || (*devNode).policy == crate::types::SERVICE_POLICY_CAPACITY as u16
        {
            if (*driverEntry).Bind.is_none() {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD002510,
                    b"device_node\0".as_ptr() as *const ::core::ffi::c_char,
                    b"driver %{public}s bind method not implement\0".as_ptr() as *const ::core::ffi::c_char,
                    (*driverEntry).moduleName,
                );
                (*devNode).devStatus = crate::types::DEVNODE_NONE as u8;
                return crate::types::HDF_ERR_INVALID_OBJECT;
            }
            ret = ((*driverEntry).Bind.unwrap())(&mut (*devNode).deviceObject);
            if ret != crate::types::HDF_SUCCESS {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD002510,
                    b"device_node\0".as_ptr() as *const ::core::ffi::c_char,
                    b"bind driver %{public}s failed\0".as_ptr() as *const ::core::ffi::c_char,
                    (*driverEntry).moduleName,
                );
                return crate::types::HDF_DEV_ERR_DEV_INIT_FAIL;
            }
        }
    }
    return crate::types::HDF_SUCCESS;
}