pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode) {
    if !devNode.is_null() {
        unsafe {
            let nodeIf: *mut crate::types::IDeviceNode = &mut (*devNode).super_;
            crate::src_hdf_device_object::HdfDeviceObjectConstruct(&mut (*devNode).deviceObject);
            (*devNode).token = crate::src_hdf_device_token::HdfDeviceTokenNewInstance();
            (*nodeIf).LaunchNode = Some(crate::src_hdf_device_node::HdfDeviceLaunchNode as unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int);
            (*nodeIf).PublishService = Some(crate::src_hdf_device_node::HdfDeviceNodePublishPublicService as unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int);
            (*nodeIf).RemoveService = Some(crate::src_hdf_device_node::HdfDeviceNodeRemoveService as unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int);
            (*nodeIf).UnlaunchNode = Some(crate::src_hdf_device_node::HdfDeviceUnlaunchNode as unsafe extern "C" fn(*mut crate::types::HdfDeviceNode));
        }
    }
}