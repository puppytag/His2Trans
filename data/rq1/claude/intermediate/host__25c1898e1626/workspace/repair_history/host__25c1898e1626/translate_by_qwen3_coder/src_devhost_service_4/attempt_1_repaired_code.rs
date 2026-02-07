pub extern "C" fn DevHostServiceAddDevice(inst: *mut crate::types::IDevHostService, deviceInfo: *const crate::types::HdfDeviceInfo) -> ::core::ffi::c_int {
    use crate::types::*;
    
    let mut ret: ::core::ffi::c_int = HDF_FAILURE;
    let mut device: *mut HdfDevice = std::ptr::null_mut();
    let mut devNode: *mut HdfDeviceNode = std::ptr::null_mut();
    let mut driver: *mut HdfDriver = std::ptr::null_mut();
    
    // CONTAINER_OF: hostService = (DevHostService*)((char*)inst - offsetof(DevHostService, super_))
    let hostService: *mut DevHostService = unsafe {
        let offset = core::mem::offset_of!(DevHostService, super_);
        (inst as *mut u8).sub(offset) as *mut DevHostService
    };
    
    let driverLoader: *mut IDriverLoader = crate::src_hdf_driver_loader::HdfDriverLoaderGetInstance();
    
    if inst.is_null() || deviceInfo.is_null() || driverLoader.is_null() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, input param is null\0".as_ptr() as *const _) };
        return ret;
    }
    
    let get_driver_fn = unsafe { (*driverLoader).GetDriver };
    if get_driver_fn.is_none() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, input param is null\0".as_ptr() as *const _) };
        return ret;
    }
    
    let device_id_raw = unsafe { (*deviceInfo).deviceId };
    let device_id_extracted: u16 = ((device_id_raw >> 8) & ((1 << 16) - 1)) as u16;
    
    device = crate::src_devhost_service::DevHostServiceQueryOrAddDevice(hostService, device_id_extracted);
    
    if device.is_null() || unsafe { (*device).super_.Attach }.is_none() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, device or Attach func is null\0".as_ptr() as *const _) };
        return HDF_DEV_ERR_NO_DEVICE;
    }
    
    let get_device_node_fn = unsafe { (*device).super_.GetDeviceNode };
    if let Some(f) = get_device_node_fn {
        devNode = unsafe { f(&mut (*device).super_, device_id_raw) };
    }
    
    if !devNode.is_null() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, device already exist\0".as_ptr() as *const _) };
        return HDF_ERR_DEVICE_BUSY;
    }
    
    let module_name = unsafe { (*deviceInfo).moduleName };
    if let Some(f) = get_driver_fn {
        driver = unsafe { f(module_name) };
    }
    
    if driver.is_null() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device %s, get driver failed\0".as_ptr() as *const _) };
        ret = HDF_DEV_ERR_NODATA;
        let dev_nodes_head = unsafe { &(*device).devNodes };
        let is_empty = unsafe { (*dev_nodes_head).next == dev_nodes_head as *const _ as *mut _ };
        if is_empty {
            crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
        }
        return ret;
    }
    
    devNode = crate::src_hdf_device_node::HdfDeviceNodeNewInstance(deviceInfo, driver);
    if devNode.is_null() {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, create devNode failed\0".as_ptr() as *const _) };
        if let Some(reclaim) = unsafe { (*driverLoader).ReclaimDriver } {
            unsafe { reclaim(driver) };
        }
        return HDF_DEV_ERR_NO_MEMORY;
    }
    
    unsafe {
        (*devNode).hostService = hostService;
        (*devNode).device = device;
        (*devNode).driver = driver;
    }
    
    if let Some(attach_fn) = unsafe { (*device).super_.Attach } {
        ret = unsafe { attach_fn(&mut (*device).super_, devNode) };
    }
    
    if ret != HDF_SUCCESS {
        unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_ERROR, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"failed to add device, attach devNode failed\0".as_ptr() as *const _) };
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(devNode);
        let dev_nodes_head = unsafe { &(*device).devNodes };
        let is_empty = unsafe { (*dev_nodes_head).next == dev_nodes_head as *const _ as *mut _ };
        if is_empty {
            crate::src_devhost_service::DevHostServiceFreeDevice(hostService, device);
        }
        return ret;
    }
    
    unsafe { crate::compat::HiLogPrint(LOG_CORE, LOG_DEBUG, 0xD002510, b"devhost_service\0".as_ptr() as *const _, b"%s add device success\0".as_ptr() as *const _) };
    HDF_SUCCESS
}