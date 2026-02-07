pub extern "C" fn SvcMgrIoserviceUnRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener) -> i32 {
    if self_.is_null() || listener.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }

    // CONTAINER_OF(self, struct SvcMgrIoservice, svcmgr) using memoffset-style approach
    let svcmgr_offset = {
        let base: usize = 0x1000;
        let base_ptr = base as *const crate::types::SvcMgrIoservice;
        let field_ptr = unsafe { &(*base_ptr).svcmgr as *const _ as usize };
        field_ptr - base
    };
    let svcmgrInst = (self_ as *mut u8).wrapping_sub(svcmgr_offset) as *mut crate::types::SvcMgrIoservice;

    // CONTAINER_OF(listener, struct IoServiceStatusListener, svcstatListener)
    let svcstatListener_offset = {
        let base: usize = 0x1000;
        let base_ptr = base as *const crate::types::IoServiceStatusListener;
        let field_ptr = unsafe { &(*base_ptr).svcstatListener as *const _ as usize };
        field_ptr - base
    };
    let listenerInst = (listener as *mut u8).wrapping_sub(svcstatListener_offset) as *mut crate::types::IoServiceStatusListener;

    let ret = unsafe {
        crate::compat::HdfDeviceUnregisterEventListener(
            (*svcmgrInst).iosvc,
            &mut (*listenerInst).ioservListener as *mut _
        )
    };
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }

    let mut ret = ret;
    unsafe {
        if crate::compat::HdfIoserviceGetListenerCount((*svcmgrInst).iosvc as *const _) == 0 {
            ret = crate::src_svcmgr_ioservice::UnSetListenClass(svcmgrInst, (*listenerInst).deviceClass);
        }
    }

    ret
}