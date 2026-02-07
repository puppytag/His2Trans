fn HdfDeviceNodePublishService(devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    let mut status = crate::types::HDF_SUCCESS;
    if devNode.is_null() {
        return status;
    }
    unsafe {
        if (*devNode).policy as u32 == crate::types::SERVICE_POLICY_NONE ||
            (!(*devNode).servName.is_null() && libc::strlen((*devNode).servName as *const i8) == 0) {
            return status;
        }
        let nodeIf = &mut (*devNode).super_ as *mut crate::types::IDeviceNode;
        if (*devNode).policy as u32 == crate::types::SERVICE_POLICY_PUBLIC ||
            (*devNode).policy as u32 == crate::types::SERVICE_POLICY_CAPACITY {
            if let Some(f) = (*nodeIf).PublishService {
                status = f(devNode);
            }
        }
        if status == crate::types::HDF_SUCCESS {
            status = crate::src_hdf_device_node::HdfDeviceNodePublishLocalService(devNode);
        }
    }
    status
}