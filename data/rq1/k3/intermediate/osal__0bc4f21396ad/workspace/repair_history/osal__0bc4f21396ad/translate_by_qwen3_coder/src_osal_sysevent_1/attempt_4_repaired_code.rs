fn HdfSysEventNotifierGetInstance() -> *mut crate::types::HdfSysEventNotifier {
    static mut HDF_SYS_EVENT_NOTIFIER: *mut crate::types::HdfSysEventNotifier = std::ptr::null_mut();
    unsafe {
        if !HDF_SYS_EVENT_NOTIFIER.is_null() {
            return HDF_SYS_EVENT_NOTIFIER;
        }
        let notifier = libc::calloc(1, std::mem::size_of::<crate::types::HdfSysEventNotifier>()) as *mut crate::types::HdfSysEventNotifier;
        if notifier.is_null() {
            return std::ptr::null_mut();
        }
        let ret = crate::compat::OsalMutexInit(std::ptr::addr_of_mut!((*notifier).mutex) as *mut crate::compat::OsalMutex);
        if ret != 0 {
            libc::free(notifier as *mut libc::c_void);
            return std::ptr::null_mut();
        }
        let head = std::ptr::addr_of_mut!((*notifier).notifyNodeList);
        (*head).next = head;
        (*head).prev = head;
        HDF_SYS_EVENT_NOTIFIER = notifier;
        notifier
    }
}