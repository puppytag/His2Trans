pub extern "C" fn DevSvcManagerClntSubscribeService(svcName: *const ::core::ffi::c_char, callback: crate::types::SubscriberCallback) -> ::core::ffi::c_int {
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    let mut serviceManager: *mut crate::types::IDevSvcManager = std::ptr::null_mut();
    if devSvcMgrClnt.is_null() || svcName.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devsvc_manager_clnt\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, client or svcName is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        serviceManager = (*devSvcMgrClnt).devSvcMgrIf;
    }
    if serviceManager.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devsvc_manager_clnt\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, method not implement\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        if let Some(func) = (*serviceManager).SubscribeService {
            return func(serviceManager, svcName, callback);
        } else {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devsvc_manager_clnt\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to subscribe service, method not implement\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return crate::types::HDF_FAILURE;
        }
    }
}