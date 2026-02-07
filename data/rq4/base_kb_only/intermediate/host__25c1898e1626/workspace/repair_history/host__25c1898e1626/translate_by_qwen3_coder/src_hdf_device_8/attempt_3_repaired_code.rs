pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    unsafe {
        (*device).super_.Attach = Some(std::mem::transmute::<
            extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int,
            unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int
        >(HdfDeviceAttach));
        (*device).super_.Detach = Some(std::mem::transmute::<
            extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int,
            unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int
        >(HdfDeviceDetach));
        (*device).super_.DetachWithDevid = Some(std::mem::transmute::<
            extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> ::core::ffi::c_int,
            unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> ::core::ffi::c_int
        >(HdfDeviceDetachWithDevid));
        (*device).super_.GetDeviceNode = Some(std::mem::transmute::<
            extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> *mut crate::types::HdfDeviceNode,
            unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> *mut crate::types::HdfDeviceNode
        >(HdfDeviceGetDeviceNode));

        let head = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*head).next = head;
        (*head).prev = head;
    }
}