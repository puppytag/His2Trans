pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr)
    // Calculate offset manually to avoid null pointer dereference
    let svcmgr_offset = core::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgrInst = unsafe {
        (self_ as *mut u8).sub(svcmgr_offset) as *mut crate::types::SvcMgrIoservice
    };

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstatListener_offset = core::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener);
    let listenerInst = unsafe {
        (listener as *mut u8).sub(svcstatListener_offset) as *mut crate::types::IoServiceStatusListener
    };

    let ret = unsafe {
        crate::compat::HdfDeviceUnregisterEventListener(
            (*svcmgrInst).iosvc as *mut _,
            &mut (*listenerInst).ioservListener as *mut _
        )
    };

    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    let mut ret = ret;
    unsafe {
        if crate::compat::HdfIoserviceGetListenerCount((*svcmgrInst).iosvc as *const _) == 0 {
            ret = UnSetListenClass(svcmgrInst, (*listenerInst).deviceClass);
        }
    }

    ret
}