pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr)
    let svcmgr_offset = unsafe {
        &(*(std::ptr::null::<crate::types::SvcMgrIoservice>())).svcmgr as *const _ as usize
    };
    let svcmgrInst = (self_ as *mut u8).wrapping_sub(svcmgr_offset) as *mut crate::types::SvcMgrIoservice;

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstatListener_offset = unsafe {
        &(*(std::ptr::null::<crate::types::IoServiceStatusListener>())).svcstatListener as *const _ as usize
    };
    let listenerInst = (listener as *mut u8).wrapping_sub(svcstatListener_offset) as *mut crate::types::IoServiceStatusListener;

    let iosvc = unsafe { (*svcmgrInst).iosvc };
    let ioservListener_ptr = unsafe { &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener };

    let ret = unsafe { crate::compat::HdfDeviceUnregisterEventListener(iosvc, ioservListener_ptr) };
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    let mut ret = ret;
    if unsafe { crate::compat::HdfIoserviceGetListenerCount(iosvc) } == 0 {
        let deviceClass = unsafe { (*listenerInst).deviceClass };
        ret = crate::src_svcmgr_ioservice::UnSetListenClass(svcmgrInst, deviceClass);
    }

    ret
}