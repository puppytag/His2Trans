pub extern "C" fn HdfDeviceGetServiceName(deviceObject: *const crate::types::HdfDeviceObject) -> *const ::core::ffi::c_char {
    use crate::types::*;
    
    if deviceObject.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to get service name, deviceObject is invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return std::ptr::null();
    }
    
    // Calculate offset of deviceObject field within HdfDeviceNode using offset_of approach
    // Use MaybeUninit to avoid null pointer dereference error
    let offset = {
        let dummy = std::mem::MaybeUninit::<HdfDeviceNode>::uninit();
        let base_ptr = dummy.as_ptr();
        unsafe {
            let field_ptr = std::ptr::addr_of!((*base_ptr).deviceObject);
            (field_ptr as *const u8).offset_from(base_ptr as *const u8) as usize
        }
    };
    
    let devNode = unsafe {
        (deviceObject as *const u8).sub(offset) as *mut HdfDeviceNode
    };
    
    // Return servName from the device node
    unsafe { (*devNode).servName as *const ::core::ffi::c_char }
}