pub extern "C" fn HdfDeviceSubscribeService(deviceObject: *mut crate::types::HdfDeviceObject, serviceName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> i32 {
    use crate::types::*;
    use crate::compat::*;
    
    if deviceObject.is_null() || serviceName.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, serviceName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_FAILURE;
    }
    
    // HDF_SLIST_CONTAINER_OF macro expansion: get HdfDeviceNode from deviceObject field
    // Use offset_of pattern without dereferencing null pointer
    let devNode: *mut HdfDeviceNode = unsafe {
        let dummy: std::mem::MaybeUninit<HdfDeviceNode> = std::mem::MaybeUninit::uninit();
        let base_ptr = dummy.as_ptr();
        let field_ptr = std::ptr::addr_of!((*base_ptr).deviceObject);
        let offset = (field_ptr as *const u8).offset_from(base_ptr as *const u8) as usize;
        (deviceObject as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    let hostService: *mut DevHostService = unsafe { (*devNode).hostService };
    
    if hostService.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, hostService is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return HDF_FAILURE;
    }
    
    let devId = unsafe { (*devNode).devId };
    let observer = unsafe { &mut (*hostService).observer as *mut HdfServiceObserver };
    
    unsafe {
        crate::src_hdf_service_observer::HdfServiceObserverSubscribeService(observer, serviceName, devId, callback)
    }
}