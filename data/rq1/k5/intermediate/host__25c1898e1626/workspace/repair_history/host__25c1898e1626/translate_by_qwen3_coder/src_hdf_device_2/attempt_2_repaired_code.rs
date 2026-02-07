fn FindUsableDevNodeId(device: *mut crate::types::HdfDevice) -> crate::types::devid_t {
    let mut node_id: u16 = 129;
    let mut find: bool = false;
    if device.is_null() {
        return node_id as crate::types::devid_t;
    }
    unsafe {
        let devid_index = (*device).devidIndex;
        while node_id <= devid_index {
            find = false;
            let dev_nodes_head = &(*device).devNodes as *const crate::types::DListHead;
            let mut entry_ptr = (*dev_nodes_head).next;
            while entry_ptr != dev_nodes_head as *mut crate::types::DListHead {
                let dev_node = (entry_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode;
                let dev_id = (*dev_node).devId;
                if (dev_id & ((1 << 8) - 1)) as u16 == node_id {
                    find = true;
                    break;
                }
                entry_ptr = (*entry_ptr).next;
            }
            if !find {
                return node_id as crate::types::devid_t;
            }
            node_id += 1;
        }
    }
    node_id as crate::types::devid_t
}