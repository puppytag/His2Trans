fn HdfDeviceUnlaunchNode(devNode: *mut crate::types::HdfDeviceNode) {
    let mut driverEntry: *const crate::types::HdfDriverEntry = std::ptr::null();
    let mut driverLoader: *mut crate::types::IDriverLoader = std::ptr::null_mut();
    if devNode.is_null() {
        return;
    }
    unsafe {
        if (*devNode).devStatus as u32 != crate::types::DEVNODE_LAUNCHED {
            return;
        }
        if !(*devNode).driver.is_null() {
            driverEntry = (*(*devNode).driver).entry;
        }
        if !driverEntry.is_null() && !(*driverEntry).Release.is_none() {
            if let Some(f) = (*driverEntry).Release {
                f(&mut (*devNode).deviceObject);
            }
        }
        if (*devNode).servStatus {
            if let Some(f) = (*devNode).super_.RemoveService {
                f(devNode);
            }
        }
        crate::src_devmgr_service_clnt::DevmgrServiceClntDetachDevice((*devNode).devId);
        driverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
        if !driverLoader.is_null() {
            if let Some(f) = (*driverLoader).ReclaimDriver {
                f((*devNode).driver);
            }
            (*devNode).driver = std::ptr::null_mut();
        } else {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "device_node\0".as_ptr() as *const i8,
                "failed to get driver loader\0".as_ptr() as *const i8,
            );
        }
        (*devNode).devStatus = crate::types::DEVNODE_INITED as u8;
    }
}