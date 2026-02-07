pub extern "C" fn HdfDeviceNodeConstruct(devNode: *mut crate::types::HdfDeviceNode) {
    if !devNode.is_null() {
        unsafe {
            let nodeIf = &mut (*devNode).super_;
            crate::src_hdf_device_object::HdfDeviceObjectConstruct(&mut (*devNode).deviceObject);
            (*devNode).token = crate::src_hdf_device_token::HdfDeviceTokenNewInstance();
            nodeIf.LaunchNode = Some(HdfDeviceLaunchNode);
            nodeIf.PublishService = Some(HdfDeviceNodePublishPublicService);
            nodeIf.RemoveService = Some(HdfDeviceNodeRemoveService);
            nodeIf.UnlaunchNode = Some(HdfDeviceUnlaunchNode);
        }
    }
}