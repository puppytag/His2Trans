fn FindUsableDevNodeId(device: *mut crate::types::HdfDevice) -> crate::types::devid_t {
    let mut nodeId: u16 = 129;
    let mut find: bool;
    let mut devNode: *mut crate::types::HdfDeviceNode;
    
    // Calculate offset of entry field in HdfDeviceNode using MaybeUninit
    let entry_offset = {
        let dummy = std::mem::MaybeUninit::<crate::types::HdfDeviceNode>::uninit();
        let base_ptr = dummy.as_ptr();
        unsafe { std::ptr::addr_of!((*base_ptr).entry) as usize - base_ptr as usize }
    };
    
    unsafe {
        while nodeId <= (*device).devidIndex {
            find = false;
            
            // DLIST_FOR_EACH_ENTRY expansion:
            let list_head = &(*device).devNodes as *const crate::types::DListHead;
            let mut current = (*list_head).next;
            
            while current != list_head as *mut crate::types::DListHead {
                devNode = (current as *mut u8).sub(entry_offset) as *mut crate::types::HdfDeviceNode;
                
                // Check if DEVNODEID(devNode->devId) == nodeId
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