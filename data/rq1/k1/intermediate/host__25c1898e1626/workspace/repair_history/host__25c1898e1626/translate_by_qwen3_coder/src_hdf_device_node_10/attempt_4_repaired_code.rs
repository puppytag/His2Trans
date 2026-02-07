pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode) {
    if !devNode.is_null() {
        unsafe {
            let nodeIf = &mut (*devNode).super_;
            crate::src_hdf_device_object::HdfDeviceObjectConstruct(&mut (*devNode).deviceObject);
            (*devNode).token = crate::src_hdf_device_token::HdfDeviceTokenNewInstance();
            nodeIf.LaunchNode = Some(crate::compat::HdfDeviceLaunchNode);
            nodeIf.PublishService = Some(crate::compat::HdfDeviceNodePublishPublicService);
            nodeIf.RemoveService = Some(crate::compat::HdfDeviceNodeRemoveService);
            nodeIf.UnlaunchNode = Some(crate::compat::HdfDeviceUnlaunchNode);
        }
    }
}