fn HdfDeviceGetDeviceNode(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t) -> *mut crate::types::HdfDeviceNode {
    unsafe {
        let super_offset = std::mem::offset_of!(crate::types::HdfDevice, super_);
        let dev = (device as *mut u8).sub(super_offset) as *mut crate::types::HdfDevice;
        
        let entry_offset = std::mem::offset_of!(crate::types::HdfDeviceNode, entry);
        let dev_nodes_head = &(*dev).devNodes as *const crate::types::DListHead;
        
        let mut current = (*dev_nodes_head).next;
        
        while current != dev_nodes_head as *mut crate::types::DListHead {
            let dev_node = (current as *mut u8).sub(entry_offset) as *mut crate::types::HdfDeviceNode;
            
            if (*dev_node).devId == devid {
                return dev_node;
            }
            
            current = (*current).next;
        }
        
        std::ptr::null_mut()
    }
}