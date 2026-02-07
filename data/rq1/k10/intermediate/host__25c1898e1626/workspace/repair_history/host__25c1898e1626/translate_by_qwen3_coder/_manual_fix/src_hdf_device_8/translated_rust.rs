pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    unsafe {
        (*device).super_.Attach = Some(crate::src_hdf_device::HdfDeviceAttach as unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> i32);
        (*device).super_.Detach = Some(crate::compat::HdfDeviceDetach);
        (*device).super_.DetachWithDevid = Some(crate::src_hdf_device::HdfDeviceDetachWithDevid as unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> i32);
        (*device).super_.GetDeviceNode = Some(crate::src_hdf_device::HdfDeviceGetDeviceNode as unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> *mut crate::types::HdfDeviceNode);
        (*device).devNodes.next = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*device).devNodes.prev = &mut (*device).devNodes as *mut crate::types::DListHead;
    }
}