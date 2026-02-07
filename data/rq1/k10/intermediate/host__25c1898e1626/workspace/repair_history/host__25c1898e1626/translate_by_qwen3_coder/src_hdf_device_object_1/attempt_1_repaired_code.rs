pub extern "C" fn HdfDeviceSubscribeService(deviceObject: *mut crate::types::HdfDeviceObject, serviceName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> i32 {
    let mut hostService: *mut crate::types::DevHostService = std::ptr::null_mut();
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    if deviceObject.is_null() || serviceName.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"device_object\0".as_ptr() as *const _, b"failed to subscribe service, serviceName is null\0".as_ptr() as *const _) };
        return crate::types::HDF_FAILURE as i32;
    }
    unsafe {
        let __mptr = deviceObject as *const crate::types::HdfDeviceObject;
        devNode = (__mptr as *const u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode;
    }
    unsafe {
        hostService = (*devNode).hostService;
    }
    if hostService.is_null() {
        return crate::types::HDF_FAILURE as i32;
    }
    unsafe {
        return crate::src_hdf_service_observer::HdfServiceObserverSubscribeService(&mut (*hostService).observer, serviceName, (*devNode).devId, callback);
    }
}