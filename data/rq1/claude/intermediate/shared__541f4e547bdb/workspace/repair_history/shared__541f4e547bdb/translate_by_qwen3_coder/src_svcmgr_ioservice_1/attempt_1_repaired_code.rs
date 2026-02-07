fn ProcessListenClass(svcmgrInst: *mut crate::types::SvcMgrIoservice, devClass: u16, cmdId: i32) -> i32 {
    use crate::compat::*;
    use crate::types::*;
    
    let data = unsafe { HdfSbufObtainDefaultSize() };
    if data.is_null() {
        return HDF_ERR_MALLOC_FAIL;
    }
    
    let _ = unsafe { HdfSbufWriteUint16(data, devClass) };
    
    unsafe {
        let iosvc = (*svcmgrInst).iosvc;
        if iosvc.is_null() {
            HdfSbufRecycle(data);
            return HDF_ERR_INVALID_OBJECT;
        }
        
        let dispatcher = (*iosvc).dispatcher;
        if dispatcher.is_null() {
            HdfSbufRecycle(data);
            return HDF_ERR_INVALID_OBJECT;
        }
        
        let dispatch_fn = (*dispatcher).Dispatch;
        if dispatch_fn.is_none() {
            HdfSbufRecycle(data);
            return HDF_ERR_INVALID_OBJECT;
        }
        
        let ret = dispatch_fn.unwrap()(
            iosvc as *mut HdfObject,
            cmdId as ::core::ffi::c_int,
            data,
            std::ptr::null_mut()
        );
        
        HdfSbufRecycle(data);
        ret
    }
}