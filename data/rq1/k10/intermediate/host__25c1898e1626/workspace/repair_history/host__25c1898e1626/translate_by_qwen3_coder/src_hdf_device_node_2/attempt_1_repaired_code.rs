fn HdfDeviceNodePublishService(devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    let mut status = crate::types::HDF_SUCCESS;
    unsafe {
        if devNode.is_null() {
            return status;
        }
        let policy = (*devNode).policy;
        let serv_name = (*devNode).servName;
        if policy as u32 == crate::types::SERVICE_POLICY_NONE ||
            (!serv_name.is_null() && libc::strlen(serv_name) == 0) {
            return status;
        }
        let node_if = &mut (*devNode).super_ as *mut crate::types::IDeviceNode;
        if policy as u32 == crate::types::SERVICE_POLICY_PUBLIC ||
            policy as u32 == crate::types::SERVICE_POLICY_CAPACITY {
            if let Some(f) = (*node_if).PublishService {
                status = f(devNode);
            }
        }
        if status == crate::types::HDF_SUCCESS {
            status = crate::src_hdf_device_node::HdfDeviceNodePublishLocalService(devNode);
        }
    }
    status
}