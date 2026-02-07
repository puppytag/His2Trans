fn DevHostServiceFindDevice(hostService: *mut crate::types::DevHostService, deviceId: u16) -> *mut crate::types::HdfDevice {
    let mut device: *mut crate::types::HdfDevice = std::ptr::null_mut();
    if hostService.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            "devhost_service\0".as_ptr() as *const _,
            "failed to find driver, hostService is null\0".as_ptr() as *const _,
        ) };
        return std::ptr::null_mut();
    }
    unsafe {
        let devices_ptr = &(*hostService).devices as *const crate::types::DListHead;
        let mut node_ptr = (*devices_ptr).next;
        while node_ptr != devices_ptr as *mut crate::types::DListHead {
            device = (node_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize)) as *mut crate::types::HdfDevice;
            if ((((*device).deviceId >> 8) & ((1 << 16) - 1)) as u16) == deviceId {
                return device;
            }
            node_ptr = (*node_ptr).next;
        }
    }
    std::ptr::null_mut()
}