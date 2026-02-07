fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32 {
    let data = crate::compat::HdfSbufObtainDefaultSize();
    if data.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL as i32;
    }
    let write_ok = crate::compat::HdfSbufWriteUint16(data, devClass);
    if !write_ok {
        crate::compat::HdfSbufRecycle(data);
        return crate::types::HDF_ERR_INVALID_OBJECT as i32;
    }
    let iosvc = unsafe { (*svcmgrInst).iosvc };
    if iosvc.is_null() {
        crate::compat::HdfSbufRecycle(data);
        return crate::types::HDF_ERR_INVALID_OBJECT as i32;
    }
    let dispatcher = unsafe { (*iosvc).dispatcher };
    if dispatcher.is_null() {
        crate::compat::HdfSbufRecycle(data);
        return crate::types::HDF_ERR_INVALID_OBJECT as i32;
    }
    let dispatch_fn = unsafe { (*dispatcher).Dispatch };
    if dispatch_fn.is_none() {
        crate::compat::HdfSbufRecycle(data);
        return crate::types::HDF_ERR_INVALID_OBJECT as i32;
    }
    let ret = unsafe { dispatch_fn.unwrap()(iosvc as *mut crate::types::HdfObject, cmdId as i32, data, std::ptr::null_mut()) };
    crate::compat::HdfSbufRecycle(data);
    ret
}