fn FindUsableDevNodeId(device: *mut crate::types::HdfDevice) -> crate::types::devid_t {
    let mut nodeId: u16 = 129;
    let mut find: bool;
    let mut devNode: *mut crate::types::HdfDeviceNode;
    
    // Calculate offset of entry field in HdfDeviceNode using offset_of approach
    // This is equivalent to: (char *)&((struct HdfDeviceNode *)0)->entry
    let entry_offset = core::mem::offset_of!(crate::types::HdfDeviceNode, entry);
    
    unsafe {
        while nodeId <= (*device).devidIndex {
            find = false;
            
            // Get the head of the list
            let head = &(*device).devNodes as *const crate::types::DListHead;
            let mut current = (*head).next;
            
            // Iterate through the list
            while current != head as *mut crate::types::DListHead {
                // Get HdfDeviceNode from DListHead entry
                devNode = (current as *mut u8).sub(entry_offset) as *mut crate::types::HdfDeviceNode;
                
                // Check if this node's devId matches nodeId
                // DEVNODEID extracts lower 8 bits: (devId & ((1 << 8) - 1))
                let dev_node_id = ((*devNode).devId as u32) & ((1u32 << 8) - 1);
                if dev_node_id == nodeId as u32 {
                    find = true;
                    break;
                }
                
                current = (*current).next;
            }
            
            if !find {
                return nodeId as crate::types::devid_t;
            }
            
            nodeId += 1;
        }
    }
    
    nodeId as crate::types::devid_t
}