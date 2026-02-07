pub extern "C" fn HdfPmSetMode(deviceObject: *mut crate::types::HdfDeviceObject, mode: u32) {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut token: *mut crate::types::PowerStateToken = std::ptr::null_mut();
    if deviceObject.is_null() || mode > crate::types::HDF_POWER_MODE_MAX as u32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD002510, b"device_object\0".as_ptr() as *const _, b"%{public}s: input param is invalid\0".as_ptr() as *const _, b"HdfPmSetMode\0".as_ptr() as *const _) };
        return;
    }
    unsafe {
        let __mptr = deviceObject as *const crate::types::HdfDeviceObject;
        let offset = (&(*(std::ptr::null::<crate::types::HdfDeviceNode>())).deviceObject) as *const _ as usize;
        devNode = (__mptr as *const u8).offset(-(offset as isize)) as *mut crate::types::HdfDeviceNode;
    }
}