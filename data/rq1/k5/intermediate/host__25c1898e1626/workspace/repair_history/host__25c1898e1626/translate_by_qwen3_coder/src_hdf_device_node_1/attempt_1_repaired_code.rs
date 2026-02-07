fn HdfDeviceNodePublishLocalService(devNode: *mut crate::types::HdfDeviceNode) -> i32 {
    if devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to publish local service, device is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        if (*devNode).hostService.is_null() {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_node\0".as_ptr() as *const _, b"failed to publish local service, host service is null\0".as_ptr() as *const _);
            return crate::types::HDF_FAILURE;
        }
        let host_service = (*devNode).hostService;
        let observer = &mut (*host_service).observer;
        let svc_name = (*devNode).servName as *const ::core::ffi::c_char;
        let device_id = (*devNode).devId;
        let policy = (*devNode).policy;
        let service = (*devNode).deviceObject.service as *mut crate::types::HdfObject;
        crate::src_hdf_service_observer::HdfServiceObserverPublishService(observer, svc_name, device_id, policy, service)
    }
}