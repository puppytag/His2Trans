fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    let sbuf = unsafe { crate::compat::HdfSbufObtain(std::mem::size_of::<u64>() as u32) };
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
    let ret = unsafe { ((*dispatcher).Dispatch)(object as *const _ as *mut _, 1, sbuf, std::ptr::null_mut()) };
    if ret != crate::types::HDF_SUCCESS as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(
            3u32,
            3u32,
            0xD002510,
            b"usysevent\0".as_ptr() as *const _,
            b"failed to finish sysevent, %{public}d\0".as_ptr() as *const _,
            ret,
        ) };
    }
    unsafe { crate::compat::HdfSbufRecycle(sbuf) };
    ret
}