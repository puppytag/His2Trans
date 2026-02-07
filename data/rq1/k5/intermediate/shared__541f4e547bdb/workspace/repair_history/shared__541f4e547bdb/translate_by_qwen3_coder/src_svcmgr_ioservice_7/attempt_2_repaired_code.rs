pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice {
    unsafe {
        let svcmgrInst = libc::malloc(std::mem::size_of::<crate::types::SvcMgrIoservice>()) as *mut crate::types::SvcMgrIoservice;
        if svcmgrInst.is_null() {
            return std::ptr::null_mut();
        }
        std::ptr::write_bytes(svcmgrInst, 0, 1);
        (*svcmgrInst).iosvc = crate::src_hdf_io_service::HdfIoServiceBind(b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char);
        if (*svcmgrInst).iosvc.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char,
                b"ioserivce %{public}s not exist\0".as_ptr() as *const ::core::ffi::c_char,
                b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char,
            );
            libc::free(svcmgrInst as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        crate::src_svcmgr_ioservice::SvcMgrIoserviceConstruct(std::ptr::addr_of_mut!((*svcmgrInst).svcmgr) as *mut crate::types::ISvcMgrIoservice);
        std::ptr::addr_of_mut!((*svcmgrInst).svcmgr) as *mut crate::types::ISvcMgrIoservice
    }
}