pub extern "C" fn HdfDeviceSubscribeService(deviceObject: *mut crate::types::HdfDeviceObject, serviceName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> i32 {
    let mut hostService: *mut crate::types::DevHostService = std::ptr::null_mut();
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() || serviceName.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD002510, "device_object\0".as_ptr() as *const _, "failed to subscribe service, serviceName is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE as i32;
    }
    unsafe {
        let offset = std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize;
        devNode = (deviceObject as *mut u8).offset(-offset) as *mut crate::types::HdfDeviceNode;
    }
    if devNode.is_null() {
        return crate::types::HDF_FAILURE as i32;
    }
    unsafe {
        hostService = (*devNode).hostService;
    }
    if hostService.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD002510, "device_object\0".as_ptr() as *const _, "failed to subscribe service, hostService is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE as i32;
    }
    unsafe {
        crate::src_hdf_service_observer::HdfServiceObserverSubscribeService(&mut (*hostService).observer as *mut crate::types::HdfServiceObserver, serviceName, (*devNode).devId, callback)
    }
}