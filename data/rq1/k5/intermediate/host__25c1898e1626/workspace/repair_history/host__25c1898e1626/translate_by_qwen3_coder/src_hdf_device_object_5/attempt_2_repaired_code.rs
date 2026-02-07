pub extern "C" fn HdfPmAcquireDevice(deviceObject: *mut crate::types::HdfDeviceObject) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut tokenIf: *mut crate::types::IPowerStateToken = std::ptr::null_mut();
    if deviceObject.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const _,
                b"HdfPmAcquireDevice input param is invalid\0".as_ptr() as *const _,
            )
        };
        return;
    }
    unsafe {
        let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize;
        devNode = (deviceObject as *mut u8).offset(-offset) as *mut crate::types::HdfDeviceNode;
    }
    if !devNode.is_null() {
        unsafe {
            tokenIf = (*devNode).powerToken as *mut crate::types::IPowerStateToken;
        }
    }
    if !tokenIf.is_null() {
        unsafe {
            if let Some(f) = (*tokenIf).AcquireWakeLock {
                f(tokenIf);
            }
        }
    }
}