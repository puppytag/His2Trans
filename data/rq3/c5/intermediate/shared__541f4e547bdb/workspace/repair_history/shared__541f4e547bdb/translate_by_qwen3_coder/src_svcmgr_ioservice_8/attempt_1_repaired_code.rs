pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    
    // CONTAINER_OF(svcmgr, struct SvcMgrIoservice, svcmgr)
    // Calculate offset of svcmgr field within SvcMgrIoservice
    let offset = std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgrInst = unsafe {
        (svcmgr as *mut u8).offset(-(offset as isize)) as *mut crate::types::SvcMgrIoservice
    };
    
    unsafe {
        // Cast the iosvc field to the expected type for HdfIoServiceRecycle
        crate::src_hdf_io_service::HdfIoServiceRecycle((*svcmgrInst).iosvc as *mut crate::types::HdfIoService);
        crate::compat::OsalMemFree(svcmgrInst as *mut ::core::ffi::c_void);
    }
}