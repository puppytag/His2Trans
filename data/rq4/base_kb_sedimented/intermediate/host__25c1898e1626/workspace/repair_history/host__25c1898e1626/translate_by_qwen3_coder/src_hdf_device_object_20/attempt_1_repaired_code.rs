pub extern "C" fn HdfDeviceObjectSetInterfaceDesc(dev: *mut crate::types::HdfDeviceObject, interfaceDesc: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if dev.is_null() || interfaceDesc.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    
    // CONTAINER_OF macro: devNode = (struct HdfDeviceNode *)((char *)(dev) - (char *)&((struct HdfDeviceNode *)0)->deviceObject)
    // Calculate offset of deviceObject field in HdfDeviceNode using offset_of approach
    let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    let devNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    };
    
    unsafe {
        (*devNode).interfaceDesc = crate::compat::HdfStringCopy(interfaceDesc);
        if (*devNode).interfaceDesc.is_null() {
            crate::types::HDF_ERR_MALLOC_FAIL
        } else {
            crate::types::HDF_SUCCESS
        }
    }
}