pub extern "C" fn HdfDeviceNodeNewInstance(deviceInfo: *const crate::types::HdfDeviceInfo, driver: *mut crate::types::HdfDriver) -> *mut crate::types::HdfDeviceNode {
    use crate::compat::*;
    use crate::types::*;
    
    let mut devNode: *mut HdfDeviceNode = std::ptr::null_mut();
    
    if deviceInfo.is_null() {
        return std::ptr::null_mut();
    }
    
    devNode = unsafe { HdfObjectManagerGetObject(HDF_OBJECT_ID_DEVICE_SERVICE as i32) as *mut HdfDeviceNode };
    if devNode.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        (*devNode).driver = driver;
        (*devNode).devId = (*deviceInfo).deviceId;
        (*devNode).permission = (*deviceInfo).permission;
        (*devNode).policy = (*deviceInfo).policy;
        (*(*devNode).token).devid = (*deviceInfo).deviceId;
        (*devNode).servName = HdfStringCopy((*deviceInfo).svcName);
        (*(*devNode).token).servName = HdfStringCopy((*deviceInfo).svcName);
        (*(*devNode).token).deviceName = HdfStringCopy((*deviceInfo).deviceName);
        
        if (*devNode).servName.is_null() {
            crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
            return std::ptr::null_mut();
        }
        
        (*devNode).deviceObject.property = HcsGetNodeByMatchAttr(HdfGetHcsRootNode(), (*deviceInfo).deviceMatchAttr);
        if (*devNode).deviceObject.property.is_null() {
            let _ = HiLogPrint(
                LOG_CORE as u32,
                LOG_DEBUG as u32,
                0xD002510u32,
                b"device_node\0".as_ptr() as *const ::core::ffi::c_char,
                b"node %{public}s property empty, match attr: %{public}s\0".as_ptr() as *const ::core::ffi::c_char,
                (*deviceInfo).moduleName,
                (*deviceInfo).deviceMatchAttr,
            );
        }
        
        (*devNode).devStatus = DEVNODE_INITED as u8;
    }
    
    devNode
}