pub extern "C" fn DevSvcManagerClntAddService(service: *mut crate::types::HdfDeviceObject, servinfo: *const crate::types::HdfServiceInfo) -> ::core::ffi::c_int {
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if devSvcMgrClnt.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "devsvc_manager_clnt\0".as_ptr() as *const i8,
                "failed to add service, client is null\0".as_ptr() as *const i8,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        if (*servinfo).devClass >= crate::types::DEVICE_CLASS_MAX as u16 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "devsvc_manager_clnt\0".as_ptr() as *const i8,
                "failed to add service, invalid class\0".as_ptr() as *const i8,
            );
            return crate::types::HDF_FAILURE;
        }
    }
    let serviceManager = unsafe { (*devSvcMgrClnt).devSvcMgrIf };
    if serviceManager.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "devsvc_manager_clnt\0".as_ptr() as *const i8,
                "serviceManager AddService function is null\0".as_ptr() as *const i8,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let add_service = unsafe { (*serviceManager).AddService };
    if let Some(f) = add_service {
        unsafe { f(serviceManager, service, servinfo) }
    } else {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "devsvc_manager_clnt\0".as_ptr() as *const i8,
                "serviceManager AddService function is null\0".as_ptr() as *const i8,
            );
        }
        crate::types::HDF_FAILURE
    }
}