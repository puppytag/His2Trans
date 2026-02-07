pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr)
    let svcmgr_offset = std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr);
    let svcmgrInst = unsafe {
        (self_ as *mut u8).offset(-(svcmgr_offset as isize)) as *mut crate::types::SvcMgrIoservice
    };

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstat_offset = std::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener);
    let listenerInst = unsafe {
        (listener as *mut u8).offset(-(svcstat_offset as isize)) as *mut crate::types::IoServiceStatusListener
    };

    let iosvc_ptr = unsafe { (*svcmgrInst).iosvc };
    let listener_ptr = unsafe { &mut (*listenerInst).ioservListener as *mut crate::types::HdfDevEventlistener };
    
    let ret = unsafe {
        crate::compat::HdfDeviceUnregisterEventListener(
            iosvc_ptr as *mut crate::types::HdfIoService,
            listener_ptr as *mut crate::types::HdfDevEventlistener
        )
    };

    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    let mut ret = ret;
    unsafe {
        if crate::compat::HdfIoserviceGetListenerCount(iosvc_ptr as *const crate::types::HdfIoService) == 0 {
            ret = crate::src_svcmgr_ioservice::UnSetListenClass(svcmgrInst, (*listenerInst).deviceClass);
        }
    }

    ret
}