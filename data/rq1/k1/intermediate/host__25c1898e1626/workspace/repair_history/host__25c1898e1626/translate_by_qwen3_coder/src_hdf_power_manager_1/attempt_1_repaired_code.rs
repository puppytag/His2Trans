fn HdfPmTaskQueueInstance() -> *mut crate::types::PmTaskQueue {
    static mut PM_TASK_QUEUE: crate::types::PmTaskQueue = unsafe { ::core::mem::MaybeUninit::zeroed().assume_init() };
    unsafe { &mut PM_TASK_QUEUE }
}