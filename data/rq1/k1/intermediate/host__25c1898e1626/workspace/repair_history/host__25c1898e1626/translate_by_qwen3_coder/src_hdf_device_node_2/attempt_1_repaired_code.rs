fn HdfDeviceNodePublishService(devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    let mut status = crate::types::HDF_SUCCESS;
    if devNode.is_null() {
        return status;
    }
    let policy = unsafe { (*devNode).policy };
    let serv_name = unsafe { (*devNode).servName };
    if policy as u32 == crate::types::SERVICE_POLICY_NONE ||
        (!serv_name.is_null() && unsafe { libc::strlen(serv_name as *const i8) } == 0) {
        return status;
    }
    let node_if = unsafe { &(*devNode).super_ };
    if policy as u32 == crate::types::SERVICE_POLICY_PUBLIC || policy as u32 == crate::types::SERVICE_POLICY_CAPACITY {
        if let Some(f) = node_if.PublishService {
            status = unsafe { f(devNode) };
        }
    }
    if status == crate::types::HDF_SUCCESS {
        status = crate::src_hdf_device_node::HdfDeviceNodePublishLocalService(devNode);
    }
    status
}