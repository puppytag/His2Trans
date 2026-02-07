pub extern "C" fn DevmgrServiceClntAttachDevice(deviceToken: *mut crate::types::IHdfDeviceToken) -> ::core::ffi::c_int {
    let mut devMgrSvcIf: *mut crate::types::IDevmgrService = std::ptr::null_mut();
    let inst = crate::src_devmgr_service_clnt::DevmgrServiceClntGetInstance();
    if inst.is_null() || unsafe { (*inst).devMgrSvcIf.is_null() } {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "devmgr_service_clnt\0".as_ptr() as *const _,
                "devmgr client failed to attach device, inst is null\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    devMgrSvcIf = unsafe { (*inst).devMgrSvcIf };
    if unsafe { (*devMgrSvcIf).AttachDevice.is_none() } {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "devmgr_service_clnt\0".as_ptr() as *const _,
                "devmgr client failed to attach device, dmsOps->AttachDevice is nul\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    unsafe {
        if let Some(f) = (*devMgrSvcIf).AttachDevice {
            f(devMgrSvcIf, deviceToken)
        } else {
            crate::types::HDF_FAILURE
        }
    }
}