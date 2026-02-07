pub extern "C" fn HdfDeviceObjectUpdate(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    if dev.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    
    // CONTAINER_OF macro: devNode = (struct HdfDeviceNode *)((char *)(dev) - (char *)&((struct HdfDeviceNode *)0)->deviceObject)
    // Use offset_of pattern without dereferencing null pointer
    let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    let devNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    };
    
    // Initialize HdfServiceInfo from devNode (inline HdfServiceInfoInit)
    let mut servInfo: crate::types::HdfServiceInfo = unsafe { std::mem::zeroed() };
    unsafe {
        servInfo.servName = (*devNode).servName as *const ::core::ffi::c_char;
        servInfo.servInfo = (*devNode).servInfo;
        servInfo.devClass = (*devNode).deviceObject.deviceClass as u16;
        servInfo.devId = (*devNode).devId;
        servInfo.interfaceDesc = (*devNode).interfaceDesc as *const ::core::ffi::c_char;
    }
    
    unsafe {
        crate::src_devsvc_manager_clnt::DevSvcManagerClntUpdateService(
            &mut (*devNode).deviceObject as *mut crate::types::HdfDeviceObject,
            &servInfo as *const crate::types::HdfServiceInfo
        )
    }
}