pub extern "C" fn SvcMgrIoserviceRegSvcStatListener(self_: *mut crate::types::ISvcMgrIoservice, listener: *mut crate::types::ServiceStatusListener, deviceClass: u16) -> i32 {
    if self_.is_null() || listener.is_null() || deviceClass >= crate::types::DEVICE_CLASS_MAX as u16 {
        return crate::types::HDF_ERR_INVALID_PARAM as i32;
    }
    let svcmgr_inst = unsafe { (self_ as *mut u8).offset(-(std::mem::offset_of!(crate::types::SvcMgrIoservice, svcmgr) as isize)) as *mut crate::types::SvcMgrIoservice };
    let listener_inst = unsafe { (listener as *mut u8).offset(-(std::mem::offset_of!(crate::types::IoServiceStatusListener, svcstatListener) as isize)) as *mut crate::types::IoServiceStatusListener };
    unsafe {
        (*listener_inst).deviceClass = deviceClass;
    }
    let ret = crate::src_svcmgr_ioservice::SetListenClass(svcmgr_inst, deviceClass);
    if ret != crate::types::HDF_SUCCESS as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as u32, crate::types::LOG_ERROR as u32, 0xD002510, b"HDF_LOG_TAG\0".as_ptr() as *const i8, b"failed to set listen class\0".as_ptr() as *const i8);
        }
        return ret;
    }
    unsafe {
        crate::compat::HdfDeviceRegisterEventListener((*svcmgr_inst).iosvc as *mut crate::types::HdfIoService, &mut (*listener_inst).ioservListener)
    }
}