pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode) {
    if !devNode.is_null() {
        unsafe {
            let nodeIf: *mut crate::types::IDeviceNode = &mut (*devNode).super_;
            crate::src_hdf_device_object::HdfDeviceObjectConstruct(&mut (*devNode).deviceObject);
            (*devNode).token = crate::src_hdf_device_token::HdfDeviceTokenNewInstance();
            
            unsafe extern "C" fn launch_node_wrapper(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
                crate::src_hdf_device_node::HdfDeviceLaunchNode(devNode)
            }
            unsafe extern "C" fn publish_service_wrapper(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
                crate::src_hdf_device_node::HdfDeviceNodePublishPublicService(devNode)
            }
            unsafe extern "C" fn remove_service_wrapper(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
                crate::src_hdf_device_node::HdfDeviceNodeRemoveService(devNode)
            }
            unsafe extern "C" fn unlaunch_node_wrapper(devNode: *mut crate::types::HdfDeviceNode) {
                crate::src_hdf_device_node::HdfDeviceUnlaunchNode(devNode)
            }
            
            (*nodeIf).LaunchNode = Some(launch_node_wrapper);
            (*nodeIf).PublishService = Some(publish_service_wrapper);
            (*nodeIf).RemoveService = Some(remove_service_wrapper);
            (*nodeIf).UnlaunchNode = Some(unlaunch_node_wrapper);
        }
    }
}