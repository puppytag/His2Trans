fn DeInitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) {
    unsafe {
        if (*notifier).keventIoService.is_null() {
            return;
        }
        let _ = crate::compat::HdfDeviceUnregisterEventListener((*notifier).keventIoService, &mut (*notifier).ioServiceListener);
        crate::compat::HdfIoServiceRecycle((*notifier).keventIoService);
        (*notifier).keventIoService = std::ptr::null_mut();
    }
}