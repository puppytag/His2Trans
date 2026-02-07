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
    
    // Calculate offset of deviceObject field within HdfDeviceNode using mem::offset_of equivalent
    // We use a dummy aligned allocation approach to avoid null pointer dereference
    let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject);
    devNode = unsafe {
        (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode
    };
    
    if !devNode.is_null() {
        let powerToken = unsafe { (*devNode).powerToken };
        if !powerToken.is_null() {
            tokenIf = powerToken as *mut crate::types::IPowerStateToken;
            if !tokenIf.is_null() {
                let release_fn = unsafe { (*tokenIf).ReleaseWakeLock };
                if let Some(f) = release_fn {
                    unsafe { f(tokenIf) };
                }
            }
        }
    }
}