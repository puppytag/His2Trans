pub extern "C" fn DevmgrServiceClntAttachDeviceHost(hostId: u16, hostService: *mut crate::types::IDevHostService) -> ::core::ffi::c_int {
    let mut devMgrSvcIf: *mut crate::types::IDevmgrService = std::ptr::null_mut();
    let inst = crate::src_devmgr_service_clnt::DevmgrServiceClntGetInstance();
    if inst.is_null() || unsafe { (*inst).devMgrSvcIf }.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "devmgr_service_clnt\0".as_ptr() as *const _,
                "failed to attach device host, get device manager service client is null\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    devMgrSvcIf = unsafe { (*inst).devMgrSvcIf };
    if unsafe { (*devMgrSvcIf).AttachDeviceHost }.is_none() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "devmgr_service_clnt\0".as_ptr() as *const _,
                "failed to attach device host, attach device host function is null\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        if let Some(f) = (*devMgrSvcIf).AttachDeviceHost {
            f(devMgrSvcIf, hostId, hostService)
        } else {
            crate::types::HDF_FAILURE
        }
    }
}