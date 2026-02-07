pub extern "C" fn SvcMgrIoserviceRelease(svcmgr: *mut crate::types::ISvcMgrIoservice) {
    if svcmgr.is_null() {
        return;
    }
    let svcmgr_inst = unsafe {
        (svcmgr as *mut u8).offset(-(std::mem::size_of::<crate::types::ISvcMgrIoservice>() as isize))
            as *mut crate::types::SvcMgrIoservice
    };
    unsafe {
        crate::src_hdf_io_service::HdfIoServiceRecycle((*svcmgr_inst).iosvc as *mut crate::types::HdfIoService);
        libc::free(svcmgr_inst as *mut libc::c_void);
    }
}