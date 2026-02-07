fn HdfDeviceGetDeviceNode(device: *mut crate::types::IHdfDevice, devid: crate::types::devid_t) -> *mut crate::types::HdfDeviceNode {
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let dev: *mut crate::types::HdfDevice = unsafe {
        (device as *mut u8).offset(-((&(*(std::ptr::null::<crate::types::HdfDevice>())).super_) as *const crate::types::IHdfDevice as isize)) as *mut crate::types::HdfDevice
    };
    unsafe {
        devNode = ((*dev).devNodes.next as *mut u8).offset(-((&(*(std::ptr::null::<crate::types::HdfDeviceNode>())).entry) as *const crate::types::DListHead as isize)) as *mut crate::types::HdfDeviceNode;
        while !std::ptr::eq(&(*devNode).entry, &(*dev).devNodes) {
            if (*devNode).devId == devid {
                return devNode;
            }
            devNode = ((*devNode).entry.next as *mut u8).offset(-((&(*(std::ptr::null::<crate::types::HdfDeviceNode>())).entry) as *const crate::types::DListHead as isize)) as *mut crate::types::HdfDeviceNode;
        }
    }
    std::ptr::null_mut()
}