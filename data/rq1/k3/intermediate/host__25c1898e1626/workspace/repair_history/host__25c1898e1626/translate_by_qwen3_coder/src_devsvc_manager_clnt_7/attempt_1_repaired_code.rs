pub extern "C" fn DevSvcManagerClntRemoveService(svcName: *const ::core::ffi::c_char) {
    let devSvcMgrClnt = crate::src_devsvc_manager_clnt::DevSvcManagerClntGetInstance();
    if devSvcMgrClnt.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD002510,
            b"devsvc_manager_clnt\0".as_ptr() as *const ::core::ffi::c_char,
            b"failed to remove service, devSvcMgrClnt is null\0".as_ptr() as *const ::core::ffi::c_char,
        ) };
        return;
    }
    let serviceManager = unsafe { (*devSvcMgrClnt).devSvcMgrIf };
    if serviceManager.is_null() {
        return;
    }
    let remove_service = unsafe { (*serviceManager).RemoveService };
    if let Some(f) = remove_service {
        unsafe { f(serviceManager, svcName, std::ptr::null()) };
    }
}