pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    
    // CONTAINER_OF macro: get SvcMgrIoservice from ISvcMgrIoservice pointer
    // offset = &((struct SvcMgrIoservice *)0)->svcmgr
    let offset = unsafe {
        &(*(std::ptr::null::<crate::types::SvcMgrIoservice>())).svcmgr as *const _ as usize
    };
    let svcmgr_inst = unsafe {
        (svcmgr as *mut u8).sub(offset) as *mut crate::types::SvcMgrIoservice
    };
    
    unsafe {
        crate::src_hdf_io_service::HdfIoServiceRecycle((*svcmgr_inst).iosvc);
        crate::compat::OsalMemFree(svcmgr_inst as *mut ::core::ffi::c_void);
    }
}