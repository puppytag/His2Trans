pub extern "C" fn SvcMgrIoserviceRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener, deviceClass: u16) -> i32 {
    if self_.is_null() || listener.is_null() || deviceClass as u32 >= crate::types::DEVICE_CLASS_MAX {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr) - use offset_of! macro or manual calculation
    let svcmgr_offset = core::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgrInst = (self_ as *mut u8).wrapping_sub(svcmgr_offset) as *mut crate::types::SvcMgrIoservice;

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstatListener_offset = core::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener);
    let listenerInst = (listener as *mut u8).wrapping_sub(svcstatListener_offset) as *mut crate::types::IoServiceStatusListener;

    unsafe {
        (*listenerInst).deviceClass = deviceClass;
    }

    let ret = crate::src_svcmgr_ioservice::SetListenClass(svcmgrInst, deviceClass);
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    unsafe {
        crate::compat::HdfDeviceRegisterEventListener(
            (*svcmgrInst).iosvc,
            &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener
        )
    }
}