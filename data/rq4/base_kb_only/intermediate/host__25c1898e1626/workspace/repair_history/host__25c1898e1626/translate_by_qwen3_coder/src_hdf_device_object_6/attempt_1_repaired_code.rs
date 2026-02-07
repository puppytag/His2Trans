pub extern "C" fn HdfPmReleaseDevice(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut tokenIf: *mut crate::types::IPowerStateToken = std::ptr::null_mut();
    
    if deviceObject.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmReleaseDevice input param is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    // HDF_SLIST_CONTAINER_OF macro expansion: get HdfDeviceNode from deviceObject field
    // Use offset_of pattern without dereferencing null pointer
    unsafe {
        let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
        devNode = (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode;
        
        if !devNode.is_null() {
            let powerToken = (*devNode).powerToken;
            if !powerToken.is_null() {
                tokenIf = powerToken as *mut crate::types::IPowerStateToken;
                if !tokenIf.is_null() {
                    if let Some(release_fn) = (*tokenIf).ReleaseWakeLock {
                        release_fn(tokenIf);
                    }
                }
            }
        }
    }
}