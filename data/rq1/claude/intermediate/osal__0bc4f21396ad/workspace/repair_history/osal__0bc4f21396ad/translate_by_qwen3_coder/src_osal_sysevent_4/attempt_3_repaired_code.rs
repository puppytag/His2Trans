fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    unsafe {
        if (*notifier).keventIoService.is_null() {
            (*notifier).keventIoService = crate::compat::HdfIoServiceBind(b"hdf_kevent\0".as_ptr() as *const i8);
        }
        if (*notifier).keventIoService.is_null() {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE!(),
                crate::types::LOG_ERROR!(),
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            return crate::types::HDF_DEV_ERR_NO_DEVICE!() as i32;
        }

        (*notifier).ioServiceListener.onReceive = Some(OnKEventReceived);
        (*notifier).ioServiceListener.priv_ = notifier as *mut ::core::ffi::c_void;
        let ret = crate::compat::HdfDeviceRegisterEventListener(
            (*notifier).keventIoService,
            &mut (*notifier).ioServiceListener as *mut crate::types::HdfDevEventlistener,
        );
        if ret != crate::types::HDF_SUCCESS as i32 {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE!(),
                crate::types::LOG_ERROR!(),
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %{public}s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            crate::compat::HdfIoServiceRecycle((*notifier).keventIoService);
            (*notifier).keventIoService = std::ptr::null_mut();
        }

        ret
    }
}