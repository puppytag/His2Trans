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
    
    // HDF_SLIST_CONTAINER_OF macro expansion: get HdfDeviceNode from deviceObject field
    // Calculate offset using MaybeUninit to avoid null pointer dereference
    unsafe {
        let dummy = std::mem::MaybeUninit::<crate::types::HdfDeviceNode>::uninit();
        let base_ptr = dummy.as_ptr();
        let field_ptr = std::ptr::addr_of!((*base_ptr).deviceObject);
        let offset = (field_ptr as *const u8).offset_from(base_ptr as *const u8) as usize;
        
        devNode = (deviceObject as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode;
        
        token = (*devNode).powerToken;
        if !token.is_null() {
            (*token).mode = mode;
        }
    }
}