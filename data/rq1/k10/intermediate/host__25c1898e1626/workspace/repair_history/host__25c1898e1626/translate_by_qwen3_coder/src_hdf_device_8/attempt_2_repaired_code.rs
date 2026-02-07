pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    unsafe {
        (*device).super_.Attach = Some(crate::src_hdf_device::HdfDeviceAttach as _);
        (*device).super_.Detach = Some(crate::compat::HdfDeviceDetach as _);
        (*device).super_.DetachWithDevid = Some(crate::src_hdf_device::HdfDeviceDetachWithDevid as _);
        (*device).super_.GetDeviceNode = Some(crate::src_hdf_device::HdfDeviceGetDeviceNode as _);
        (*device).devNodes.next = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*device).devNodes.prev = &mut (*device).devNodes as *mut crate::types::DListHead;
    }
}