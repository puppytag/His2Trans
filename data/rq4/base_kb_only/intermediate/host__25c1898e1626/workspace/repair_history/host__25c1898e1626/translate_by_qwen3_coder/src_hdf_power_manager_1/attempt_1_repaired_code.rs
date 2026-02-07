fn HdfPmTaskQueueInstance() -> *mut crate::types::PmTaskQueue {
    static mut PM_TASK_QUEUE: *mut crate::types::PmTaskQueue = std::ptr::null_mut();
    unsafe {
        if PM_TASK_QUEUE.is_null() {
            PM_TASK_QUEUE = libc::malloc(1) as *mut crate::types::PmTaskQueue;
            if !PM_TASK_QUEUE.is_null() {
                std::ptr::write_bytes(PM_TASK_QUEUE, 0, 1);
            }
        }
        PM_TASK_QUEUE
    }
}