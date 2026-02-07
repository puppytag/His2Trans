fn HdfDeviceGetDeviceNode(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t) -> *mut crate::types::HdfDeviceNode {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if device.is_null() {
        return std::ptr::null_mut();
    }
    let dev = unsafe { (device as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, super_) as isize)) as *mut crate::types::HdfDevice };
    let dev_nodes_ptr = unsafe { std::ptr::addr_of_mut!((*dev).devNodes) };
    let mut entry = unsafe { (*dev_nodes_ptr).next };
    while entry != dev_nodes_ptr {
        devNode = unsafe { (entry as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode };
        if unsafe { (*devNode).devId } == devid {
            return devNode;
        }
        entry = unsafe { (*entry).next };
    }
    std::ptr::null_mut()
}