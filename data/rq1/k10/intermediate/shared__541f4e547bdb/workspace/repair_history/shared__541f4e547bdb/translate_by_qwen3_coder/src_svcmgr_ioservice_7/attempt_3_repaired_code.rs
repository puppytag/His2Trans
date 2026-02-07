pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice {
    let svcmgrInst = unsafe { libc::malloc(std::mem::size_of::<crate::types::SvcMgrIoservice>()) as *mut crate::types::SvcMgrIoservice };
    if svcmgrInst.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        std::ptr::write_bytes(svcmgrInst, 0, 1);
    }
    unsafe {
        (*svcmgrInst).iosvc = crate::src_hdf_io_service::HdfIoServiceBind(b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char) as *mut crate::types::HdfIoService;
    }
    unsafe {
        if (*svcmgrInst).iosvc.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"ioserivce %{public}s not exist\0".as_ptr() as *const ::core::ffi::c_char,
                b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            libc::free(svcmgrInst as *mut libc::c_void);
            return std::ptr::null_mut();
        }
    }
    unsafe {
        crate::src_svcmgr_ioservice::SvcMgrIoserviceConstruct(std::ptr::addr_of_mut!((*svcmgrInst).svcmgr) as *mut crate::types::ISvcMgrIoservice);
    }
    unsafe {
        std::ptr::addr_of_mut!((*svcmgrInst).svcmgr) as *mut crate::types::ISvcMgrIoservice
    }
}