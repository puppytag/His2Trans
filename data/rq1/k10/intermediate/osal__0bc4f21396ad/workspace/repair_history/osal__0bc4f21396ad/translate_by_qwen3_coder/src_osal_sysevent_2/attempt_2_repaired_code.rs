fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    let sbuf = unsafe { crate::compat::HdfSbufObtain(std::mem::size_of::<u64>() as u32) };
    if sbuf.is_null() {
        return -1;
    }
    let sync_token = unsafe { (*event).syncToken };
    if !unsafe { crate::compat::HdfSbufWriteUint64(sbuf, sync_token) } {
        unsafe { crate::compat::HdfSbufRecycle(sbuf) };
        return -1;
    }
    let dispatcher = unsafe { (*service).dispatcher };
    let object = unsafe { &(*service).object };
    let ret = unsafe { ((*dispatcher).Dispatch)(object as *const _ as *mut _, 1, sbuf, std::ptr::null_mut()) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(
            3,
            3,
            0xD002510,
            b"usysevent\0".as_ptr() as *const _,
            b"failed to finish sysevent, %{public}d\0".as_ptr() as *const _,
            ret,
        ) };
    }
    unsafe { crate::compat::HdfSbufRecycle(sbuf) };
    ret
}