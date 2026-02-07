pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode) {
    if !devNode.is_null() {
        unsafe {
            let nodeIf: *mut crate::types::IDeviceNode = &mut (*devNode).super_;
            crate::src_hdf_device_object::HdfDeviceObjectConstruct(&mut (*devNode).deviceObject);
            (*devNode).token = crate::src_hdf_device_token::HdfDeviceTokenNewInstance();
            (*nodeIf).LaunchNode = Some(HdfDeviceLaunchNode as unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int);
            (*nodeIf).PublishService = Some(HdfDeviceNodePublishPublicService as unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int);
            (*nodeIf).RemoveService = Some(HdfDeviceNodeRemoveService as unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int);
            (*nodeIf).UnlaunchNode = Some(HdfDeviceUnlaunchNode as unsafe extern "C" fn(*mut crate::types::HdfDeviceNode));
        }
    }
}