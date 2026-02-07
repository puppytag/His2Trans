pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode) {
    if !devNode.is_null() {
        unsafe {
            let nodeIf: *mut crate::types::IDeviceNode = &mut (*devNode).super_;
            crate::src_hdf_device_object::HdfDeviceObjectConstruct(&mut (*devNode).deviceObject);
            (*devNode).token = crate::src_hdf_device_token::HdfDeviceTokenNewInstance();
            (*nodeIf).LaunchNode = Some(std::mem::transmute::<*const (), unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int>(HdfDeviceLaunchNode as *const ()));
            (*nodeIf).PublishService = Some(std::mem::transmute::<*const (), unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int>(HdfDeviceNodePublishPublicService as *const ()));
            (*nodeIf).RemoveService = Some(std::mem::transmute::<*const (), unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int>(HdfDeviceNodeRemoveService as *const ()));
            (*nodeIf).UnlaunchNode = Some(std::mem::transmute::<*const (), unsafe extern "C" fn(*mut crate::types::HdfDeviceNode)>(HdfDeviceUnlaunchNode as *const ()));
        }
    }
}