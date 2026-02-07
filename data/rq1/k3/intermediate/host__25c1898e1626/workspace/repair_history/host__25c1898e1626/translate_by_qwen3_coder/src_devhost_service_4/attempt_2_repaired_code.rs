pub extern "C" fn DevHostServiceAddDevice(inst: *mut crate::types::IDevHostService, deviceInfo: *const crate::types::HdfDeviceInfo) -> ::core::ffi::c_int {
    let mut ret = crate::types::HDF_FAILURE;
    let mut device: *mut crate::types::HdfDevice = std::ptr::null_mut();
    let mut devNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut driver: *mut crate::types::HdfDriver = std::ptr::null_mut();
    let hostService: *mut crate::types::DevHostService = if !inst.is_null() {
        unsafe { (inst as *mut u8).offset(-(std::mem::offset_of!(crate::types::DevHostService, super_) as isize)) as *mut crate::types::DevHostService }
    } else {
        std::ptr::null_mut()
    };
    let driverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
    if inst.is_null() || deviceInfo.is_null() || driverLoader.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, input param is null\0".as_ptr() as *const _) };
        return ret;
    }
    let get_driver = unsafe { (*driverLoader).GetDriver };
    if get_driver.is_none() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, input param is null\0".as_ptr() as *const _) };
        return ret;
    }
    let device_id = unsafe { (*deviceInfo).deviceId };
    let device_id_shifted = ((device_id >> 8) & ((1 << 16) - 1)) as u16;
    device = crate::src_devhost_service::DevHostServiceQueryOrAddDevice(hostService, device_id_shifted);
    if device.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, device or Attach func is null\0".as_ptr() as *const _) };
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }
    let attach = unsafe { (*device).super_.Attach };
    if attach.is_none() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, device or Attach func is null\0".as_ptr() as *const _) };
        return crate::types::HDF_DEV_ERR_NO_DEVICE;
    }
    let get_device_node = unsafe { (*device).super_.GetDeviceNode };
    if let Some(f) = get_device_node {
        devNode = unsafe { f(&mut (*device).super_ as *mut crate::types::IHdfDevice, device_id) };
    }
    if !devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, device already exist\0".as_ptr() as *const _) };
        return crate::types::HDF_ERR_DEVICE_BUSY;
    }
    let module_name = unsafe { (*deviceInfo).moduleName };
    driver = unsafe { get_driver.unwrap()(module_name) };
    if driver.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device %{public}s, get driver failed\0".as_ptr() as *const _, module_name) };
        ret = crate::types::HDF_DEV_ERR_NODATA;
        unsafe {
            let dev_nodes = &(*device).devNodes;
            let is_empty = dev_nodes.next == dev_nodes as *const crate::types::DListHead as *mut crate::types::DListHead;
            if is_empty {
                crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
            }
        }
        return ret;
    }
    devNode = crate::src_hdf_device_node::HdfDeviceNodeNewInstance(deviceInfo, driver);
    if devNode.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, create devNode failed\0".as_ptr() as *const _) };
        let reclaim_driver = unsafe { (*driverLoader).ReclaimDriver };
        if let Some(f) = reclaim_driver {
            unsafe { f(driver) };
        }
        return crate::types::HDF_DEV_ERR_NO_MEMORY;
    }
    unsafe {
        (*devNode).hostService = hostService;
        (*devNode).device = device;
        (*devNode).driver = driver;
    }
    ret = unsafe { attach.unwrap()(&mut (*device).super_ as *mut crate::types::IHdfDevice, devNode) };
    if ret != crate::types::HDF_SUCCESS {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, attach devNode failed\0".as_ptr() as *const _) };
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
        unsafe {
            let dev_nodes = &(*device).devNodes;
            let is_empty = dev_nodes.next == dev_nodes as *const crate::types::DListHead as *mut crate::types::DListHead;
            if is_empty {
                crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
            }
        }
        return ret;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_DEBUG, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"%{public}s add device success\0".as_ptr() as *const _, b"DevHostServiceAddDevice\0".as_ptr() as *const _) };
    return crate::types::HDF_SUCCESS;
}