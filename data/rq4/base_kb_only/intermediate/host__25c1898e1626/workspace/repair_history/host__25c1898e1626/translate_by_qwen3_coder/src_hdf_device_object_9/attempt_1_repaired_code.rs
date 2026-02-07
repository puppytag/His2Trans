pub extern "C" fn HdfPmSetMode(deviceObject: *mut crate::types::HdfDeviceObject, mode: u32) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut token: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    
    if deviceObject.is_null() || mode > crate::types::HDF_POWER_MODE_MAX {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s: input param is invalid\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmSetMode\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    unsafe {
        // Calculate offset using a dummy aligned allocation approach
        // offset_of!(HdfDeviceNode, deviceObject)
        let dummy: std::mem::MaybeUninit<crate::types::HdfDeviceNode> = std::mem::MaybeUninit::uninit();
        let base_ptr = dummy.as_ptr();
        let field_ptr = std::ptr::addr_of!((*base_ptr).deviceObject);
        let offset = (field_ptr as usize) - (base_ptr as usize);
        
        devNode = (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode;
        
        if devNode.is_null() {
            return;
        }
        
        token = (*devNode).powerToken;
        if token.is_null() {
            return;
        }
        
        (*token).mode = mode;
    }
}