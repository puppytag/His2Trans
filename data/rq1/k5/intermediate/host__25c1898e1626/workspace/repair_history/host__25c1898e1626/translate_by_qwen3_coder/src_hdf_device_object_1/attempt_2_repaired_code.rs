pub extern "C" fn HdfDeviceSubscribeService(deviceObject: *mut crate::types::HdfDeviceObject, serviceName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> i32 {
    let mut hostService: *mut crate::types::DevHostService = std::ptr::null_mut();
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() || serviceName.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "device_object\0".as_ptr() as *const ::core::ffi::c_char,
                "failed to subscribe service, serviceName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        devNode = (deviceObject as *mut ::core::ffi::c_char).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode;
        hostService = (*devNode).hostService;
    }
    if hostService.is_null() {
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        let devId = (*devNode).devId;
        return crate::src_hdf_service_observer::HdfServiceObserverSubscribeService(
            &mut (*hostService).observer as *mut crate::types::HdfServiceObserver,
            serviceName,
            devId,
            callback,
        );
    }
}