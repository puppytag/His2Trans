fn HdfDeviceNodePublishLocalService(devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    if devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to publish local service, device is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    let host_service = unsafe { (*devNode).hostService };
    if host_service.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to publish local service, host service is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        crate::src_hdf_service_observer::HdfServiceObserverPublishService(
            &mut (*host_service).observer,
            (*devNode).servName,
            (*devNode).devId,
            (*devNode).policy,
            (*devNode).deviceObject.service as *mut crate::types::HdfObject,
        )
    }
}