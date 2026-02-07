fn DevHostServiceFindDevice(hostService: *mut crate::types::DevHostService, deviceId: u16) -> *mut crate::types::HdfDevice {
    let mut device: *mut crate::types::HdfDevice = std::ptr::null_mut();
    
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
        // Calculate offset of 'node' field in HdfDevice using offset_of pattern
        let node_offset = core::mem::offset_of!(crate::types::HdfDevice, node);
        
        // Get pointer to the devices list head
        let devices_head = std::ptr::addr_of_mut!((*hostService).devices);
        
        // First element: container_of(devices->next, HdfDevice, node)
        let mut current_node = (*devices_head).next;
        
        while current_node != devices_head {
            // container_of: get HdfDevice* from DListHead*
            device = ((current_node as *mut u8).sub(node_offset)) as *mut crate::types::HdfDevice;
            
            // Extract device ID: ((device->deviceId >> 8) & ((1 << 16) - 1))
            let dev_id_field = (*device).deviceId;
            let extracted_id = ((dev_id_field >> 8) & ((1u32 << 16) - 1)) as u16;
            
            if extracted_id == deviceId {
                return device;
            }
            
            // Move to next node
            current_node = (*current_node).next;
        }
    }
    
    std::ptr::null_mut()
}