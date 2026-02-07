pub extern "C" fn HdfDeviceNodeNewInstance(deviceInfo: *const crate::types::HdfDeviceInfo, driver: *mut crate::types::HdfDriver) -> *mut crate::types::HdfDeviceNode {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceInfo.is_null() {
        return std::ptr::null_mut();
    }
    devNode = unsafe { crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVICE_SERVICE as i32) } as *mut crate::types::HdfDeviceNode;
    if devNode.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        (*devNode).driver = driver;
        (*devNode).devId = (*deviceInfo).deviceId;
        (*devNode).permission = (*deviceInfo).permission;
        (*devNode).policy = (*deviceInfo).policy;
        (*(*devNode).token).devid = (*deviceInfo).deviceId;
        (*devNode).servName = crate::compat::HdfStringCopy((*deviceInfo).svcName);
        (*(*devNode).token).servName = crate::compat::HdfStringCopy((*deviceInfo).svcName);
        (*(*devNode).token).deviceName = crate::compat::HdfStringCopy((*deviceInfo).deviceName);
    }

    if unsafe { (*devNode).servName.is_null() } {
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
        return std::ptr::null_mut();
    }

    unsafe {
        (*devNode).deviceObject.property = crate::compat::HcsGetNodeByMatchAttr(crate::compat::HdfGetHcsRootNode(), (*deviceInfo).deviceMatchAttr);
        if (*devNode).deviceObject.property.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD002510,
                b"device_node\0".as_ptr() as *const ::core::ffi::c_char,
                b"node %{public}s property empty, match attr: %{public}s\0".as_ptr() as *const ::core::ffi::c_char,
                (*deviceInfo).moduleName,
                (*deviceInfo).deviceMatchAttr,
            );
        }

        (*devNode).devStatus = crate::types::DEVNODE_INITED as u8;
    }
    devNode
}