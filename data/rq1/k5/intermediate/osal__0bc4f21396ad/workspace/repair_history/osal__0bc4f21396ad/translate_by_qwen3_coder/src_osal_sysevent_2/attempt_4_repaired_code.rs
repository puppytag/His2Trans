fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    let sbuf = unsafe { crate::compat::HdfSbufObtain(8) };
    if sbuf.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL as i32;
    }
    let sync_token = unsafe { (*event).syncToken };
    if !unsafe { crate::compat::HdfSbufWriteUint64(sbuf, sync_token) } {
        unsafe { crate::compat::HdfSbufRecycle(sbuf) };
        return crate::types::HDF_FAILURE as i32;
    }
    let dispatcher = unsafe { (*service).dispatcher };
    let object = unsafe { &(*service).object };
    let ret = if let Some(func) = unsafe { (*dispatcher).Dispatch } {
        unsafe { func(object as *const _ as *mut _, 1, sbuf, std::ptr::null_mut()) }
    } else {
        crate::types::HDF_FAILURE as i32
    };
    if ret != crate::types::HDF_SUCCESS as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                3u32,
                3u32,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to finish sysevent, %{public}d\0".as_ptr() as *const i8,
                ret,
            )
        };
    }
    unsafe { crate::compat::HdfSbufRecycle(sbuf) };
    ret
}