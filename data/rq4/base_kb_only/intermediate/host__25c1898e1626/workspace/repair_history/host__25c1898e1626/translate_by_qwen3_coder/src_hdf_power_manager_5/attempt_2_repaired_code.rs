pub extern "C" fn HdfPmTaskPut(powerToken: *mut crate::types::PowerStateToken, type_: crate::types::HDF_PM_REQUEST_TYPE) {
    use crate::src_hdf_power_manager::HdfPmTaskQueueInstance;
    
    if powerToken.is_null() {
        return;
    }
    
    let pmTaskQueue = HdfPmTaskQueueInstance();
    
    let pmRequest = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<crate::types::HdfPmRequest>() as u32)
    } as *mut crate::types::HdfPmRequest;
    
    if pmRequest.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD002510,
                b"hdf_power_manager\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s OsalMemCalloc fail\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmTaskPut\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    unsafe {
        (*pmRequest).token = powerToken;
        (*pmRequest).pmType = type_;
        (*pmRequest).task.func = std::mem::transmute::<_, crate::types::HdfTaskFunc>(crate::src_hdf_power_manager::PmTaskFunc as usize);
        
        crate::compat::HdfTaskEnqueue(std::ptr::null_mut(), &mut (*pmRequest).task);
    }
    let _ = pmTaskQueue;
}