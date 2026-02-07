pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    unsafe {
        (*device).super_.Attach = Some(crate::HdfDeviceAttach);
        (*device).super_.Detach = Some(crate::HdfDeviceDetach);
        (*device).super_.DetachWithDevid = Some(crate::HdfDeviceDetachWithDevid);
        (*device).super_.GetDeviceNode = Some(crate::HdfDeviceGetDeviceNode);
        let head = &mut (*device).devNodes;
        head.next = head as *mut crate::types::DListHead;
        head.prev = head as *mut crate::types::DListHead;
    }
}