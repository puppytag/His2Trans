pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    unsafe {
        (*device).super_.Attach = Some(crate::compat::HdfDeviceAttach);
        (*device).super_.Detach = Some(crate::compat::HdfDeviceDetach);
        (*device).super_.DetachWithDevid = Some(crate::compat::HdfDeviceDetachWithDevid);
        (*device).super_.GetDeviceNode = Some(crate::compat::HdfDeviceGetDeviceNode);
        (*device).devNodes.next = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*device).devNodes.prev = &mut (*device).devNodes as *mut crate::types::DListHead;
    }
}