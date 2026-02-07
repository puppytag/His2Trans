fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    let sbuf = crate::compat::HdfSbufObtain(8);
    if sbuf.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL as i32;
    }
    let sync_token = unsafe { (*event).syncToken };
    if crate::compat::HdfSbufWriteUint64(sbuf, sync_token) == 0 {
        crate::compat::HdfSbufRecycle(sbuf);
        return crate::types::HDF_FAILURE as i32;
    }
    let dispatcher = unsafe { (*service).dispatcher };
    let object = unsafe { &(*service).object };
    let ret = unsafe { ((*dispatcher).Dispatch)(object as *const _ as *mut _, 1, sbuf, std::ptr::null_mut()) };
    if ret != crate::types::HDF_SUCCESS as i32 {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE!() as crate::types::LogType,
            crate::types::LOG_ERROR!() as crate::types::LogLevel,
            0xD002510,
            "usysevent\0".as_ptr() as *const i8,
            "failed to finish sysevent, %{public}d\0".as_ptr() as *const i8,
            ret,
        );
    }
    crate::compat::HdfSbufRecycle(sbuf);
    ret
}