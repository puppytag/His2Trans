pub extern "C" fn SvcMgrIoserviceRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener, deviceClass: u16) -> i32 {
    if self_.is_null() || listener.is_null() || deviceClass as u32 >= crate::types::DEVICE_CLASS_MAX {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let svcmgrInst = unsafe {
        (self_ as *mut u8).offset(-(std::mem::size_of::<crate::types::SvcMgrIoservice>() as isize)) as *mut crate::types::SvcMgrIoservice
    };
    let listenerInst = unsafe {
        (listener as *mut u8).offset(-(std::mem::size_of::<crate::types::IoServiceStatusListener>() as isize)) as *mut crate::types::IoServiceStatusListener
    };
    unsafe {
        (*listenerInst).deviceClass = deviceClass;
    }
    let ret = crate::src_svcmgr_ioservice::SetListenClass(svcmgrInst, deviceClass);
    if ret != crate::types::HDF_SUCCESS {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD002510, b"HDF_LOG_TAG\0".as_ptr() as *const i8, b"failed to set listen class\0".as_ptr() as *const i8);
        }
        return ret;
    }
    unsafe {
        crate::compat::HdfDeviceRegisterEventListener((*svcmgrInst).iosvc, &mut (*listenerInst).ioservListener)
    }
}