pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }
    let svcmgrInst = unsafe {
        (self_ as *mut u8).offset(-(std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr) as isize))
            as *mut crate::types::SvcMgrIoservice
    };
    let listenerInst = unsafe {
        (listener as *mut u8).offset(-(std::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener) as isize))
            as *mut crate::types::IoServiceStatusListener
    };
    let ret = unsafe {
        crate::compat::HdfDeviceUnregisterEventListener(
            (*svcmgrInst).iosvc as *mut crate::types::HdfIoService,
            &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener,
        )
    };
    if ret != crate::types::HDF_SUCCESS as i32 {
        return ret;
    }
    let count = unsafe { crate::compat::HdfIoserviceGetListenerCount((*svcmgrInst).iosvc as *const crate::types::HdfIoService) };
    if count == 0 {
        let dev_class = unsafe { (*listenerInst).deviceClass };
        return crate::src_svcmgr_ioservice::UnSetListenClass(svcmgrInst, dev_class);
    }
    ret
}