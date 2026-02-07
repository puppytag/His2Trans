pub extern "C" fn HdfDeviceGetServiceName(deviceObject: *const crate::types::HdfDeviceObject) -> *const ::core::ffi::c_char {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const _, b"failed to get service name, deviceObject is invalid\0".as_ptr() as *const _) };
        return std::ptr::null();
    }
    unsafe {
        let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize;
        devNode = (deviceObject as *const u8).offset(-offset) as *mut crate::types::HdfDeviceNode;
        if !devNode.is_null() {
            return (*devNode).servName;
        }
    }
    std::ptr::null()
}