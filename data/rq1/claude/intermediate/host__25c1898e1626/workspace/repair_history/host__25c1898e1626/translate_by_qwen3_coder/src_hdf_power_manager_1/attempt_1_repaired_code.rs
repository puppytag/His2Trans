fn HdfPmTaskQueueInstance() -> *mut crate::types::PmTaskQueue {
    static mut PM_TASK_QUEUE: std::mem::MaybeUninit<crate::types::PmTaskQueue> = std::mem::MaybeUninit::zeroed();
    unsafe { PM_TASK_QUEUE.as_mut_ptr() }
}