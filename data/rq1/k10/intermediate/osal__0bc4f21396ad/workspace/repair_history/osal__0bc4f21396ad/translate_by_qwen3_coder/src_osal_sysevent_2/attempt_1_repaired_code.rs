fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    let sbuf = unsafe { crate::compat::HdfSbufObtain(std::mem::size_of::<u64>() as crate::types::size_t) };
    if sbuf.is_null() {
        return crate::compat::HDF_ERR_MALLOC_FAIL!();
    }
    let sync_token = unsafe { (*event).syncToken };
    if unsafe { crate::compat::HdfSbufWriteUint64(sbuf, sync_token) } == 0 {
        unsafe { crate::compat::HdfSbufRecycle(sbuf) };
        return crate::compat::HDF_FAILURE;
    }
    let dispatcher = unsafe { (*service).dispatcher };
    let object = unsafe { &(*service).object };
    let ret = unsafe { (*dispatcher).Dispatch(object as *const _ as *mut _, 1, sbuf, std::ptr::null_mut()) };
    if ret != crate::compat::HDF_SUCCESS {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::compat::LOG_CORE,
            crate::compat::LOG_ERROR,
            0xD002510,
            b"usysevent\0".as_ptr() as *const _,
            b"failed to finish sysevent, %{public}d\0".as_ptr() as *const _,
            ret,
        ) };
    }
    unsafe { crate::compat::HdfSbufRecycle(sbuf) };
    ret
}