pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice {
    let svcmgrInst: *mut crate::types::SvcMgrIoservice = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::SvcMgrIoservice>() as u32) as *mut crate::types::SvcMgrIoservice
    };
    if svcmgrInst.is_null() {
        return std::ptr::null_mut();
    }

    let service_name = b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char;
    let iosvc = crate::src_hdf_io_service::HdfIoServiceBind(service_name);
    if iosvc.is_null() {
        let tag = b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char;
        let fmt = b"ioserivce %{public}s not exist\0".as_ptr() as *const ::core::ffi::c_char;
        let svc_name_log = b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char;
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510u32,
                tag,
                fmt,
                svc_name_log,
            );
            crate::compat::OsalMemFree(svcmgrInst as *mut ::core::ffi::c_void);
        }
        return std::ptr::null_mut();
    }

    unsafe {
        (*svcmgrInst).iosvc = iosvc as *mut _;
        let svcmgr_ptr = std::ptr::addr_of_mut!((*svcmgrInst).svcmgr) as *mut crate::types::ISvcMgrIoservice;
        crate::src_svcmgr_ioservice::SvcMgrIoserviceConstruct(svcmgr_ptr);
        svcmgr_ptr
    }
}