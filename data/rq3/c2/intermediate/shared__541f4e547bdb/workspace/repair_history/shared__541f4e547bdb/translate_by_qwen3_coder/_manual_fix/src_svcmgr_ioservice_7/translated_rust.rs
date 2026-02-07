pub extern "C" fn SvcMgrIoserviceGet() -> *mut crate::types::ISvcMgrIoservice {
    unsafe {
        let svcmgr_inst = crate::compat::OsalMemCalloc(
            std::mem::size_of::<crate::types::SvcMgrIoservice>() as u32
        ) as *mut crate::types::SvcMgrIoservice;
        
        if svcmgr_inst.is_null() {
            return std::ptr::null_mut();
        }
        
        let service_name = b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char;
        let iosvc = crate::src_hdf_io_service::HdfIoServiceBind(service_name);
        
        if iosvc.is_null() {
            let tag = b"HDF_LOG_TAG\0".as_ptr() as *const ::core::ffi::c_char;
            let fmt = b"ioserivce %{public}s not exist\0".as_ptr() as *const ::core::ffi::c_char;
            let svc_name_arg = b"devsvc_mgr\0".as_ptr() as *const ::core::ffi::c_char;
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                tag,
                fmt,
                svc_name_arg,
            );
            crate::compat::OsalMemFree(svcmgr_inst as *mut ::core::ffi::c_void);
            return std::ptr::null_mut();
        }
        
        (*svcmgr_inst).iosvc = iosvc;
        
        let svcmgr_ptr = &mut (*svcmgr_inst).svcmgr as *mut crate::types::ISvcMgrIoservice;
        crate::src_svcmgr_ioservice::SvcMgrIoserviceConstruct(svcmgr_ptr);
        
        svcmgr_ptr
    }
}