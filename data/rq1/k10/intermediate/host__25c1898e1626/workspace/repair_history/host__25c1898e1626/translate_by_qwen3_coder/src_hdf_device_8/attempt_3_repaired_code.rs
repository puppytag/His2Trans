pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    unsafe {
        (*device).super_.Attach = Some(std::mem::transmute(crate::src_hdf_device::HdfDeviceAttach as *const ()));
        (*device).super_.Detach = Some(std::mem::transmute(crate::compat::HdfDeviceDetach as *const ()));
        (*device).super_.DetachWithDevid = Some(std::mem::transmute(crate::src_hdf_device::HdfDeviceDetachWithDevid as *const ()));
        (*device).super_.GetDeviceNode = Some(std::mem::transmute(crate::src_hdf_device::HdfDeviceGetDeviceNode as *const ()));
        (*device).devNodes.next = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*device).devNodes.prev = &mut (*device).devNodes as *mut crate::types::DListHead;
    }
}