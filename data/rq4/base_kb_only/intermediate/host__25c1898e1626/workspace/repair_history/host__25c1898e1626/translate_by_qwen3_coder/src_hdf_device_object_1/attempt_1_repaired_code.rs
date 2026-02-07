pub extern "C" fn HdfDeviceSubscribeService(deviceObject: *mut crate::types::HdfDeviceObject, serviceName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> i32 {
    use crate::types::*;
    
    if deviceObject.is_null() || serviceName.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, serviceName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_FAILURE;
    }
    
    // Container-of macro: get HdfDeviceNode from embedded deviceObject field
    // Use offset_of pattern without dereferencing null pointer
    let devnode_offset = core::mem::offset_of!(HdfDeviceNode, deviceObject);
    let devNode: *mut HdfDeviceNode = unsafe {
        (deviceObject as *mut u8).sub(devnode_offset) as *mut HdfDeviceNode
    };
    
    let hostService: *mut DevHostService = unsafe { (*devNode).hostService };
    
    if hostService.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, hostService is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_FAILURE;
    }
    
    let observer = unsafe { &mut (*hostService).observer as *mut HdfServiceObserver };
    let devId = unsafe { (*devNode).devId };
    
    unsafe {
        crate::src_hdf_service_observer::HdfServiceObserverSubscribeService(observer, serviceName, devId, callback)
    }
}