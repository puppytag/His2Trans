fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32 {
    let data = unsafe { crate::compat::HdfSbufObtainDefaultSize() };
    if data.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }

    let _ = unsafe { crate::compat::HdfSbufWriteUint16(data, devClass) };
    
    unsafe {
        let iosvc = (*svcmgrInst).iosvc;
        if iosvc.is_null() {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        
        let dispatcher = (*iosvc).dispatcher;
        if dispatcher.is_null() {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        
        let dispatch_fn = (*dispatcher).Dispatch;
        if dispatch_fn.is_none() {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT;
        }
        
        let ret = crate::src_hdf_io_service::HdfIoServiceDispatch(
            iosvc as *mut crate::types::HdfIoService,
            cmdId as ::core::ffi::c_int,
            data,
            std::ptr::null_mut()
        );
        
        crate::compat::HdfSbufRecycle(data);
        ret
    }
}