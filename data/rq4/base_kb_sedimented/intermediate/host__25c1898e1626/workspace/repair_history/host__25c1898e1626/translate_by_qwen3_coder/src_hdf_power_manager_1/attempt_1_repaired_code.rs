fn HdfPmTaskQueueInstance() -> *mut crate::types::PmTaskQueue {
    static mut PM_TASK_QUEUE: *mut crate::types::PmTaskQueue = std::ptr::null_mut();
    unsafe { PM_TASK_QUEUE }
}