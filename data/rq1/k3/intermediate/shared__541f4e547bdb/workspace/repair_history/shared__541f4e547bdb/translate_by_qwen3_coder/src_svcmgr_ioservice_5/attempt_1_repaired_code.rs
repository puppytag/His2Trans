pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }
    unsafe {
        let svcmgrInst = (self_ as *mut u8).offset(-(std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr) as isize)) as *mut crate::types::SvcMgrIoservice;
        let listenerInst = (listener as *mut u8).offset(-(std::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener) as isize)) as *mut crate::types::IoServiceStatusListener;
        let ret = crate::compat::HdfDeviceUnregisterEventListener((*svcmgrInst).iosvc as *mut crate::types::HdfIoService, &mut (*listenerInst).ioservListener);
        if ret != crate::types::HDF_SUCCESS as i32 {
            return ret;
        }
        if crate::compat::HdfIoserviceGetListenerCount((*svcmgrInst).iosvc as *const crate::types::HdfIoService) == 0 {
            let ret = crate::src_svcmgr_ioservice::UnSetListenClass(svcmgrInst, (*listenerInst).deviceClass);
            return ret;
        }
        ret
    }
}