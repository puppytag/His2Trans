pub extern "C" fn HdfDeviceConstruct(device: *mut crate::types::HdfDevice) {
    if device.is_null() {
        return;
    }
    
    unsafe {
        (*device).super_.Attach = Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int
        >(HdfDeviceAttach as *const ()));
        
        (*device).super_.Detach = Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*mut crate::types::IHdfDevice, *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int
        >(HdfDeviceDetach as *const ()));
        
        (*device).super_.DetachWithDevid = Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> ::core::ffi::c_int
        >(HdfDeviceDetachWithDevid as *const ()));
        
        (*device).super_.GetDeviceNode = Some(std::mem::transmute::<
            *const (),
            unsafe extern "C" fn(*mut crate::types::IHdfDevice, crate::types::devid_t) -> *mut crate::types::HdfDeviceNode
        >(HdfDeviceGetDeviceNode as *const ()));
        
        let head = &mut (*device).devNodes as *mut crate::types::DListHead;
        (*head).next = head;
        (*head).prev = head;
    }
}