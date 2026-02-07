fn InitKeventIoServiceListenerLocked(notifier: *mut crate::types::HdfSysEventNotifier) -> i32 {
    const LOG_CORE: u32 = 3;
    const LOG_ERROR: u32 = 6;
    const HDF_DEV_ERR_NO_DEVICE: i32 = -207;
    const HDF_SUCCESS: i32 = 0;
    
    unsafe {
        if (*notifier).keventIoService.is_null() {
            (*notifier).keventIoService = crate::compat::HdfIoServiceBind(b"hdf_kevent\0".as_ptr() as *const i8);
        }
        if (*notifier).keventIoService.is_null() {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            return HDF_DEV_ERR_NO_DEVICE;
        }

        extern "C" fn on_kevent_wrapper(
            listener: *mut crate::types::HdfDevEventlistener,
            service: *mut crate::types::HdfIoService,
            id: u32,
            data: *mut crate::types::HdfSBuf,
        ) -> i32 {
            OnKEventReceived(listener, service, id, data)
        }
        
        (*notifier).ioServiceListener.onReceive = Some(on_kevent_wrapper);
        (*notifier).ioServiceListener.priv_ = notifier as *mut ::core::ffi::c_void;
        
        let ret = crate::compat::HdfDeviceRegisterEventListener(
            (*notifier).keventIoService,
            &mut (*notifier).ioServiceListener as *mut crate::types::HdfDevEventlistener,
        );
        
        if ret != HDF_SUCCESS {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"usysevent\0".as_ptr() as *const i8,
                b" ioservice %s is invalid\0".as_ptr() as *const i8,
                b"hdf_kevent\0".as_ptr() as *const i8,
            );
            crate::compat::HdfIoServiceRecycle((*notifier).keventIoService);
            (*notifier).keventIoService = std::ptr::null_mut();
        }

        ret
    }
}