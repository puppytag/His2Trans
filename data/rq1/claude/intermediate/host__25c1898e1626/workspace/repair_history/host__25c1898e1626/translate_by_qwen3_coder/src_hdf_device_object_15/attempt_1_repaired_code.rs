pub extern "C" fn HdfDeviceObjectUnRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    unsafe {
        // Calculate offset of deviceObject field within HdfDeviceNode using memoffset-style calculation
        // We need to compute the offset without dereferencing a null pointer
        let dummy: std::mem::MaybeUninit<crate::types::HdfDeviceNode> = std::mem::MaybeUninit::uninit();
        let base_ptr = dummy.as_ptr();
        let field_ptr = std::ptr::addr_of!((*base_ptr).deviceObject);
        let offset = (field_ptr as *const u8).offset_from(base_ptr as *const u8) as usize;
        
        let devNode = (dev as *mut u8).sub(offset) as *mut crate::types::HdfDeviceNode;
        
        if devNode.is_null() || (*devNode).device.is_null() {
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        
        let device = (*devNode).device;
        let detach_fn = (*device).super_.Detach;
        
        if let Some(detach) = detach_fn {
            detach(&mut (*device).super_ as *mut crate::types::IHdfDevice, devNode)
        } else {
            crate::types::HDF_ERR_INVALID_OBJECT
        }
    }
}