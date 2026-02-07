fn HdfDeviceUnlaunchNode(devNode: *mut crate::types::HdfDeviceNode) {
    let mut driverEntry: *const crate::types::HdfDriverEntry = std::ptr::null();
    let mut driverLoader: *mut crate::types::IDriverLoader = std::ptr::null_mut();
    if devNode.is_null() {
        return;
    }
    unsafe {
        if (*devNode).devStatus != crate::types::DEVNODE_LAUNCHED as u8 {
            return;
        }
        if !(*devNode).driver.is_null() {
            driverEntry = (*(*devNode).driver).entry;
        }
        if !driverEntry.is_null() {
            let release = (*driverEntry).Release;
            if let Some(f) = release {
                f(&mut (*devNode).deviceObject as *mut crate::types::HdfDeviceObject);
            }
        }
        if (*devNode).servStatus {
            let remove_service = (*devNode).super_.RemoveService;
            if let Some(f) = remove_service {
                let _ = f(devNode);
            }
        }
        crate::src_devmgr_service_clnt::DevmgrServiceClntDetachDevice((*devNode).devId);
        driverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
        if !driverLoader.is_null() {
            let reclaim_driver = (*driverLoader).ReclaimDriver;
            if let Some(f) = reclaim_driver {
                f((*devNode).driver);
            }
            (*devNode).driver = std::ptr::null_mut();
        } else {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"device_node\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to get driver loader\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        (*devNode).devStatus = crate::types::DEVNODE_INITED as u8;
    }
}