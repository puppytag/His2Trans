fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32 {
    let data = crate::compat::HdfSbufObtainDefaultSize();
    if data.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL as i32;
    }
    let _ = crate::compat::HdfSbufWriteUint16(data, devClass);
    unsafe {
        if (*svcmgrInst).iosvc.is_null()
            || (*(*svcmgrInst).iosvc).dispatcher.is_null()
            || (*(*(*svcmgrInst).iosvc).dispatcher).Dispatch.is_none()
        {
            crate::compat::HdfSbufRecycle(data);
            return crate::types::HDF_ERR_INVALID_OBJECT as i32;
        }
        let ret = if let Some(dispatch_fn) = (*(*(*svcmgrInst).iosvc).dispatcher).Dispatch {
            dispatch_fn(
                (*svcmgrInst).iosvc as *mut crate::types::HdfObject,
                cmdId,
                data,
                std::ptr::null_mut(),
            )
        } else {
            crate::types::HDF_ERR_INVALID_OBJECT as i32
        };
        crate::compat::HdfSbufRecycle(data);
        ret
    }
}