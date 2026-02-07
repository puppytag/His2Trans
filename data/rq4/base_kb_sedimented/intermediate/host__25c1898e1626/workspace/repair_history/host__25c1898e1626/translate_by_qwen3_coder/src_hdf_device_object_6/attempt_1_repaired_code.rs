pub extern "C" fn HdfPmReleaseDevice(deviceObject: *mut crate::types::HdfDeviceObject) {
    use crate::types::*;
    
    let mut devNode: *mut HdfDeviceNode = std::ptr::null_mut();
    let mut tokenIf: *mut IPowerStateToken = std::ptr::null_mut();
    
    if deviceObject.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmReleaseDevice input param is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    // Calculate offset of deviceObject field within HdfDeviceNode using memoffset-style approach
    unsafe {
        let dummy: std::mem::MaybeUninit<HdfDeviceNode> = std::mem::MaybeUninit::uninit();
        let base_ptr = dummy.as_ptr();
        let field_ptr = std::ptr::addr_of!((*base_ptr).deviceObject);
        let offset = (field_ptr as *const u8).offset_from(base_ptr as *const u8) as usize;
        devNode = (deviceObject as *mut u8).sub(offset) as *mut HdfDeviceNode;
    }
    
    if devNode.is_null() {
        return;
    }
    
    unsafe {
        let powerToken = (*devNode).powerToken;
        if !powerToken.is_null() {
            tokenIf = powerToken as *mut IPowerStateToken;
            if !tokenIf.is_null() {
                if let Some(release_fn) = (*tokenIf).ReleaseWakeLock {
                    release_fn(tokenIf);
                }
            }
        }
    }
}