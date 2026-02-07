fn HdfPmTaskQueueInstance() -> *mut crate::types::PmTaskQueue {
    static mut PM_TASK_QUEUE: crate::types::PmTaskQueue = unsafe { std::mem::zeroed() };
    unsafe { &mut PM_TASK_QUEUE as *mut crate::types::PmTaskQueue }
}