pub extern "C" fn SvcMgrIoserviceRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener, deviceClass: u16) -> i32 {
    if self_.is_null() || listener.is_null() || deviceClass as u32 >= crate::types::DEVICE_CLASS_MAX {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    let svcmgrInst = self_ as *mut crate::types::SvcMgrIoservice;
    let listenerInst = listener as *mut crate::types::IoServiceStatusListener;

    unsafe {
        (*listenerInst).deviceClass = deviceClass;
    }

    let ret = crate::src_svcmgr_ioservice::SetListenClass(svcmgrInst, deviceClass);
    if ret != crate::types::HDF_SUCCESS {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                b"HDF_LOG_TAG\0".as_ptr() as *const i8,
                b"failed to set listen class\0".as_ptr() as *const i8,
            );
        }
        return ret;
    }

    unsafe {
        let iosvc = (*svcmgrInst).iosvc;
        let ioserv_listener = &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener;
        crate::compat::HdfDeviceRegisterEventListener(iosvc as *mut _, ioserv_listener as *mut _)
    }
}