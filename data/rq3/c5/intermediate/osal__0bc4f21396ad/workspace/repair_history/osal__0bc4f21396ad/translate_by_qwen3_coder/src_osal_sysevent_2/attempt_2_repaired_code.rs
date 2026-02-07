fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    unsafe {
        let sbuf = crate::compat::HdfSbufObtain(std::mem::size_of::<u64>() as u32);
        
        if sbuf.is_null() {
            return crate::types::HDF_ERR_MALLOC_FAIL as i32;
        }
        
        let sync_token = (*event).syncToken;
        if crate::compat::HdfSbufWriteUint64(sbuf, sync_token) == false {
            crate::compat::HdfSbufRecycle(sbuf);
            return crate::types::HDF_FAILURE as i32;
        }
        
        let dispatcher = (*service).dispatcher;
        if dispatcher.is_null() {
            crate::compat::HdfSbufRecycle(sbuf);
            return crate::types::HDF_FAILURE as i32;
        }
        
        let dispatch_fn = (*dispatcher).Dispatch;
        let ret = if let Some(f) = dispatch_fn {
            f(&mut (*service).object as *mut _, 1, sbuf, std::ptr::null_mut())
        } else {
            crate::types::HDF_FAILURE as i32
        };
        
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE!(),
                crate::types::LOG_ERROR!(),
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to finish sysevent, %{public}d\0".as_ptr() as *const i8,
                ret,
            );
        }
        
        crate::compat::HdfSbufRecycle(sbuf);
        ret
    }
}