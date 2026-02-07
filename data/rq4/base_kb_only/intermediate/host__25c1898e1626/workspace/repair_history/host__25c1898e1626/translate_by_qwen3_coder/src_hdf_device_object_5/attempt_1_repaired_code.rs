pub extern "C" fn HdfPmAcquireDevice(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut tokenIf: *mut crate::types::IPowerStateToken = std::ptr::null_mut();
    
    if deviceObject.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmAcquireDevice input param is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    // Calculate offset of deviceObject field within HdfDeviceNode using memoffset-style approach
    unsafe {
        // Use a dummy aligned allocation to compute offset safely
        let dummy: std::mem::MaybeUninit<crate::types::HdfDeviceNode> = std::mem::MaybeUninit::uninit();
        let base_ptr = dummy.as_ptr();
        let field_ptr = std::ptr::addr_of!((*base_ptr).deviceObject);
        let offset = (field_ptr as usize) - (base_ptr as usize);
        
        devNode = (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode;
        
        if devNode.is_null() {
            return;
        }
        
        let powerToken = (*devNode).powerToken;
        if powerToken.is_null() {
            return;
        }
        
        tokenIf = powerToken as *mut crate::types::IPowerStateToken;
        
        if !tokenIf.is_null() {
            if let Some(acquire_fn) = (*tokenIf).AcquireWakeLock {
                acquire_fn(tokenIf);
            }
        }
    }
}