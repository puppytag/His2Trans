fn HdfDeviceGetDeviceNode(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t) -> *mut crate::types::HdfDeviceNode {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let dev: *mut crate::types::HdfDevice = unsafe {
        if device.is_null() {
            return std::ptr::null_mut();
        }
        (device as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, super_) as isize)) as *mut crate::types::HdfDevice
    };
    unsafe {
        if (*dev).devNodes.next.is_null() {
            return std::ptr::null_mut();
        }
        devNode = ((*dev).devNodes.next as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode;
        while !std::ptr::eq(&(*devNode).entry, &(*dev).devNodes) {
            if (*devNode).devId == devid {
                return devNode;
            }
            if (*devNode).entry.next.is_null() {
                break;
            }
            devNode = ((*devNode).entry.next as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode;
        }
    }
    std::ptr::null_mut()
}