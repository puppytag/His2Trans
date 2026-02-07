pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    
    // CONTAINER_OF: svcmgrInst = (SvcMgrIoservice*)((char*)svcmgr - offsetof(SvcMgrIoservice, svcmgr))
    // SvcMgrIoservice has field 'svcmgr' of type ISvcMgrIoservice
    // We need to compute the offset of 'svcmgr' field within SvcMgrIoservice
    let offset = std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgrInst = unsafe {
        (svcmgr as *mut u8).offset(-(offset as isize)) as *mut crate::types::SvcMgrIoservice
    };
    
    unsafe {
        // Cast iosvc to the expected type for HdfIoServiceRecycle
        HdfIoServiceRecycle((*svcmgrInst).iosvc as *mut crate::types::HdfIoService);
        crate::compat::OsalMemFree(svcmgrInst as *mut ::core::ffi::c_void);
    }
}