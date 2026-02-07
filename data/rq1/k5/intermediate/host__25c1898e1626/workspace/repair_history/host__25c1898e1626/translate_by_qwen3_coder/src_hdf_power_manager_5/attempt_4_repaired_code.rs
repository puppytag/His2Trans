pub extern "C" fn HdfPmTaskPut(powerToken: *mut crate::types::PowerStateToken, type_: crate::types::HDF_PM_REQUEST_TYPE) {
    use crate::compat::*;
    use crate::globals::*;
    if powerToken.is_null() {
        return;
    }
    let pmTaskQueue = crate::src_hdf_power_manager::HdfPmTaskQueueInstance();
    let pmRequest = unsafe { OsalMemCalloc(std::mem::size_of::<crate::types::HdfPmRequest>() as u32) as *mut crate::types::HdfPmRequest };
    if pmRequest.is_null() {
        let _ = HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD002510,
            b"hdf_power_manager\0".as_ptr() as *const _,
            b"%{public}s OsalMemCalloc fail\0".as_ptr() as *const _,
            b"HdfPmTaskPut\0".as_ptr() as *const _,
        );
        return;
    }
    unsafe {
        (*pmRequest).token = powerToken;
        (*pmRequest).pmType = type_;
        (*pmRequest).task.func = Some(crate::src_hdf_power_manager::PmTaskFunc as unsafe extern "C" fn(*mut crate::types::HdfTaskType) -> i32);
        let taskQueue = if pmTaskQueue.is_null() {
            std::ptr::null_mut()
        } else {
            (*pmTaskQueue).taskQueue
        };
        HdfTaskEnqueue(taskQueue, &mut (*pmRequest).task);
    }
}