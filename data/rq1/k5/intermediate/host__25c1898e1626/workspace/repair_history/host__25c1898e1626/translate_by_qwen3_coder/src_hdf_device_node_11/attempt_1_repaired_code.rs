pub extern "C" fn HdfDeviceNodeDestruct(devNode: *mut crate::types::HdfDeviceNode) {
    if devNode.is_null() {
        return;
    }
    let _ = unsafe { crate::compat::HiLogPrint(
        crate::types::LOG_CORE as u32,
        crate::types::LOG_INFO as u32,
        0xD002510,
        b"device_node\0".as_ptr() as *const _,
        b"release devnode %{public}s\0".as_ptr() as *const _,
        (*devNode).servName,
    ) };
    unsafe {
        match (*devNode).devStatus {
            crate::types::DEVNODE_LAUNCHED as u8 => {
                crate::src_hdf_device_node::HdfDeviceUnlaunchNode(devNode);
            }
            crate::types::DEVNODE_INITED as u8 => {
                crate::src_hdf_device_token::HdfDeviceTokenFreeInstance((*devNode).token);
                (*devNode).token = std::ptr::null_mut();
                crate::src_power_state_token::PowerStateTokenFreeInstance((*devNode).powerToken);
                (*devNode).powerToken = std::ptr::null_mut();
                crate::compat::OsalMemFree((*devNode).servName as *mut _);
                crate::compat::OsalMemFree((*devNode).servInfo as *mut _);
                crate::compat::OsalMemFree((*devNode).driverName as *mut _);
                (*devNode).servName = std::ptr::null_mut();
                (*devNode).servInfo = std::ptr::null_mut();
                (*devNode).driverName = std::ptr::null_mut();
            }
            crate::types::DEVNODE_NONE as u8 => {}
            _ => {}
        }
    }
}