fn PmTaskFunc(para: *mut crate::types::HdfTaskType) -> i32 {
    use crate::types::{HDF_FAILURE, HDF_SUCCESS, HDF_PM_REQUEST_ACQUIRE, HDF_PM_REQUEST_RELEASE};
    if para.is_null() {
        return HDF_FAILURE;
    }
    let pm_request_ptr = unsafe {
        let base = std::ptr::null::<crate::types::HdfPmRequest>();
        let task_ptr = &(*base).task as *const crate::types::HdfTaskType as *const u8;
        let base_ptr = base as *const u8;
        let offset = base_ptr.offset_from(task_ptr);
        (para as *mut u8).offset(-offset) as *mut crate::types::HdfPmRequest
    };
    let token_if = unsafe { (*pm_request_ptr).token as *mut crate::types::IPowerStateToken };
    unsafe {
        match (*pm_request_ptr).pmType {
            HDF_PM_REQUEST_ACQUIRE => {
                if !token_if.is_null() {
                    if let Some(f) = (*token_if).AcquireWakeLock {
                        f(token_if);
                    }
                }
            }
            HDF_PM_REQUEST_RELEASE => {
                if !token_if.is_null() {
                    if let Some(f) = (*token_if).ReleaseWakeLock {
                        f(token_if);
                    }
                }
            }
            _ => {}
        }
    }
    unsafe {
        crate::compat::OsalMemFree(pm_request_ptr as *mut ::core::ffi::c_void);
    }
    HDF_SUCCESS
}