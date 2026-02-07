fn DevHostServiceFindDevice(hostService: *mut crate::types::DevHostService, deviceId: u16) -> *mut crate::types::HdfDevice {
    if hostService.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to find driver, hostService is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return std::ptr::null_mut();
    }

    unsafe {
        let devices_head = &(*hostService).devices as *const crate::types::DListHead;
        let node_offset = core::mem::offset_of!(crate::types::HdfDevice, node);
        
        let mut current_node = (*devices_head).next;
        
        while current_node != devices_head as *mut crate::types::DListHead {
            let device = (current_node as *mut u8).sub(node_offset) as *mut crate::types::HdfDevice;
            
            let dev_id = (*device).deviceId;
            let extracted_id = ((dev_id >> 8) & ((1 << 16) - 1)) as u16;
            
            if extracted_id == deviceId {
                return device;
            }
            
            current_node = (*current_node).next;
        }
    }
    
    std::ptr::null_mut()
}