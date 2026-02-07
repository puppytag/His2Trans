pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    unsafe {
        (*device).super_.Attach = Some(HdfDeviceAttach as unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int);
        (*device).super_.Detach = Some(HdfDeviceDetach as unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int);
        (*device).super_.DetachWithDevid = Some(HdfDeviceDetachWithDevid as unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> ::core::ffi::c_int);
        (*device).super_.GetDeviceNode = Some(HdfDeviceGetDeviceNode as unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> *mut crate::types::HdfDeviceNode);

        let head = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*head).next = head;
        (*head).prev = head;
    }
}