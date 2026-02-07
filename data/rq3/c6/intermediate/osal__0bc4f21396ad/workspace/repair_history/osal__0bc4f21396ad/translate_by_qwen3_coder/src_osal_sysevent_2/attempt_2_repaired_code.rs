fn FinishEvent(service: *mut crate::types::HdfIoService, event: *const crate::types::HdfSysEvent) -> i32 {
    use crate::compat::*;
    use crate::types::*;
    
    let sbuf = unsafe { HdfSbufObtain(std::mem::size_of::<u64>() as u32) };
    
    if sbuf.is_null() {
        return HDF_ERR_MALLOC_FAIL as i32;
    }
    
    let sync_token = unsafe { (*event).syncToken };
    if unsafe { HdfSbufWriteUint64(sbuf, sync_token) } == false {
        unsafe { HdfSbufRecycle(sbuf) };
        return HDF_FAILURE as i32;
    }
    
    let dispatcher = unsafe { (*service).dispatcher };
    let dispatch_fn = unsafe { (*dispatcher).Dispatch };
    let object_ptr = unsafe { &mut (*service).object as *mut HdfObject };
    
    let ret = if let Some(f) = dispatch_fn {
        unsafe { f(object_ptr, 1, sbuf, std::ptr::null_mut()) }
    } else {
        HDF_FAILURE as i32
    };
    
    if ret != HDF_SUCCESS as i32 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b"failed to finish sysevent, %{public}d\0".as_ptr() as *const i8,
                ret,
            );
        }
    }
    
    unsafe { HdfSbufRecycle(sbuf) };
    ret
}