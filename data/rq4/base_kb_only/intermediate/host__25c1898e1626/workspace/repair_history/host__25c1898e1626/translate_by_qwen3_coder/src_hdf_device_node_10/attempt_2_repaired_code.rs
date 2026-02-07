pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode) {
    if !devNode.is_null() {
        unsafe {
            let nodeIf: *mut crate::types::IDeviceNode = &mut (*devNode).super_;
            crate::src_hdf_device_object::HdfDeviceObjectConstruct(&mut (*devNode).deviceObject);
            (*devNode).token = crate::src_hdf_device_token::HdfDeviceTokenNewInstance();
            (*nodeIf).LaunchNode = Some(std::mem::transmute::<extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int, unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int>(HdfDeviceLaunchNode));
            (*nodeIf).PublishService = Some(std::mem::transmute::<extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int, unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int>(HdfDeviceNodePublishPublicService));
            (*nodeIf).RemoveService = Some(std::mem::transmute::<extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int, unsafe extern "C" fn(*mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int>(HdfDeviceNodeRemoveService));
            (*nodeIf).UnlaunchNode = Some(std::mem::transmute::<extern "C" fn(*mut crate::types::HdfDeviceNode), unsafe extern "C" fn(*mut crate::types::HdfDeviceNode)>(HdfDeviceUnlaunchNode));
        }
    }
}