pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr) using offset_of! macro approach
    let svcmgr_offset = std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgrInst = (self_ as *mut u8).wrapping_sub(svcmgr_offset) as *mut crate::types::SvcMgrIoservice;

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstatListener_offset = std::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener);
    let listenerInst = (listener as *mut u8).wrapping_sub(svcstatListener_offset) as *mut crate::types::IoServiceStatusListener;

    let iosvc = unsafe { (*svcmgrInst).iosvc };
    let ioservListener_ptr = unsafe { &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener };

    let ret = unsafe { crate::compat::HdfDeviceUnregisterEventListener(iosvc, ioservListener_ptr) };
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    let mut ret = ret;
    if unsafe { crate::compat::HdfIoserviceGetListenerCount(iosvc as *const _) } == 0 {
        let deviceClass = unsafe { (*listenerInst).deviceClass };
        ret = crate::src_svcmgr_ioservice::UnSetListenClass(svcmgrInst, deviceClass);
    }

    ret
}