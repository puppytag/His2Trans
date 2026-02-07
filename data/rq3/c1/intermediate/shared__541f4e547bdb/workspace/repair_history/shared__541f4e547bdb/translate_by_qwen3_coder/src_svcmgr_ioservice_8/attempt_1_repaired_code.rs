pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    
    // CONTAINER_OF macro: get SvcMgrIoservice from ISvcMgrIoservice pointer
    // offset = offsetof(SvcMgrIoservice, svcmgr)
    let offset = std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgr_inst = unsafe {
        (svcmgr as *mut u8).sub(offset) as *mut crate::types::SvcMgrIoservice
    };
    
    unsafe {
        // Cast the iosvc field to the expected type for HdfIoServiceRecycle
        let iosvc_ptr = (*svcmgr_inst).iosvc as *mut crate::types::HdfIoService;
        crate::src_hdf_io_service::HdfIoServiceRecycle(iosvc_ptr);
        crate::compat::OsalMemFree(svcmgr_inst as *mut ::core::ffi::c_void);
    }
}