pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    unsafe {
        (*device).super_.Attach = Some(std::mem::transmute::<_, unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> i32>(crate::src_hdf_device::HdfDeviceAttach));
        (*device).super_.Detach = Some(std::mem::transmute::<_, unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> i32>(crate::src_hdf_device::HdfDeviceDetach));
        (*device).super_.DetachWithDevid = Some(std::mem::transmute::<_, unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> i32>(crate::src_hdf_device::HdfDeviceDetachWithDevid));
        (*device).super_.GetDeviceNode = Some(std::mem::transmute::<_, unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> *mut crate::types::HdfDeviceNode>(crate::src_hdf_device::HdfDeviceGetDeviceNode));
        (*device).devNodes.next = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*device).devNodes.prev = &mut (*device).devNodes as *mut crate::types::DListHead;
    }
}