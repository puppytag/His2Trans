fn DevHostServiceStartService(service: *mut crate::types::IDevHostService) -> i32 {
    let hostService = service as *mut crate::types::DevHostService;
    if hostService.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"failed to start device service, hostService is null\0".as_ptr() as *const _,
            );
        }
        return crate::types::HDF_FAILURE;
    }
    let hostId = unsafe { (*hostService).hostId };
    crate::src_devmgr_service_clnt::DevmgrServiceClntAttachDeviceHost(hostId, service)
}