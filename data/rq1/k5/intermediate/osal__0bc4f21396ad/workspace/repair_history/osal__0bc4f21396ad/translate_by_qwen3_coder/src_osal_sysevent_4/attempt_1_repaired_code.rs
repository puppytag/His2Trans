fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    unsafe {
        if (*notifier).keventIoService.is_null() {
            (*notifier).keventIoService = crate::compat::HdfIoServiceBind("hdf_kevent\0".as_ptr() as *const i8);
        }
        if (*notifier).keventIoService.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR,
                0xD002510,
                "usysevent\0".as_ptr() as *const i8,
                " ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                "hdf_kevent\0".as_ptr() as *const i8,
            );
            return crate::compat::HDF_DEV_ERR_NO_DEVICE;
        }

        (*notifier).ioServiceListener.onReceive = Some(crate::src_osal_sysevent::OnKEventReceived);
        (*notifier).ioServiceListener.priv = notifier as *mut ::core::ffi::c_void;
        let ret = crate::compat::HdfDeviceRegisterEventListener(
            (*notifier).keventIoService,
            &mut (*notifier).ioServiceListener,
        );
        if ret != crate::compat::HDF_SUCCESS {
            let _ = crate::compat::HiLogPrint(
                crate::compat::LOG_CORE!(),
                crate::compat::LOG_ERROR,
                0xD002510,
                "usysevent\0".as_ptr() as *const i8,
                " ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                "hdf_kevent\0".as_ptr() as *const i8,
            );
            crate::compat::HdfIoServiceRecycle((*notifier).keventIoService);
            (*notifier).keventIoService = std::ptr::null_mut();
        }

        ret
    }
}