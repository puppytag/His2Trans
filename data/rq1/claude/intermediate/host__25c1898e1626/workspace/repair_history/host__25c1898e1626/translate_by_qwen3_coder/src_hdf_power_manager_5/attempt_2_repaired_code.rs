pub extern "C" fn HdfPmTaskPut(powerToken: *mut crate::types::PowerStateToken, type_: crate::types::HDF_PM_REQUEST_TYPE) {
    use crate::types::*;
    
    if powerToken.is_null() {
        return;
    }
    
    let pmTaskQueue = crate::src_hdf_power_manager::HdfPmTaskQueueInstance();
    
    let pmRequest = unsafe {
        crate::compat::OsalMemCalloc(std::mem::size_of::<HdfPmRequest>() as u32) as *mut HdfPmRequest
    };
    
    if pmRequest.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xD002510,
                b"hdf_power_manager\0".as_ptr() as *const ::core::ffi::c_char,
                b"%{public}s OsalMemCalloc fail\0".as_ptr() as *const ::core::ffi::c_char,
                b"HdfPmTaskPut\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return;
    }
    
    unsafe extern "C" fn pm_task_func_wrapper(para: *mut HdfTaskType) -> i32 {
        crate::src_hdf_power_manager::PmTaskFunc(para)
    }
    
    unsafe {
        (*pmRequest).token = powerToken;
        (*pmRequest).pmType = type_;
        (*pmRequest).task.func = Some(pm_task_func_wrapper);
        crate::compat::HdfTaskEnqueue(pmTaskQueue as *mut _, &mut (*pmRequest).task as *mut HdfTaskType);
    }
}