fn DevHostServicePmNotify(service: *mut crate::types::IDevHostService, state: u32) -> i32 {
    let mut device: *mut crate::types::HdfDevice = std::ptr::null_mut();
    let mut ret: i32 = crate::types::HDF_SUCCESS;
    let host_service_ptr = if !service.is_null() {
        unsafe {
            (service as *mut u8).offset(-(std::mem::offset_of!(crate::types::DevHostService, super_) as isize))
                as *mut crate::types::DevHostService
        }
    } else {
        std::ptr::null_mut()
    };
    if host_service_ptr.is_null() {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD002510,
                "devhost_service\0".as_ptr() as *const i8,
                "failed to start device service, hostService is null\0".as_ptr() as *const i8,
            )
        };
        return crate::types::HDF_FAILURE;
    }
    let host_service = unsafe { &*host_service_ptr };
    let host_name = if !host_service.hostName.is_null() {
        unsafe { std::ffi::CStr::from_ptr(host_service.hostName).to_str().unwrap_or("") }
    } else {
        ""
    };
    let _ = unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_DEBUG,
            0xD002510,
            "devhost_service\0".as_ptr() as *const i8,
            "host(%{public}s) set power state=%{public}u\0".as_ptr() as *const i8,
            host_name.as_ptr() as *const i8,
            state,
        )
    };
    let is_wake = state == crate::types::POWER_STATE_DOZE_RESUME || state == crate::types::POWER_STATE_RESUME;
    if is_wake {
        let mut node = unsafe { (*host_service_ptr).devices.prev };
        while !node.is_null() && node != &(*host_service_ptr).devices as *const crate::types::DListHead as *mut crate::types::DListHead {
            device = unsafe {
                (node as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize))
                    as *mut crate::types::HdfDevice
            };
            if unsafe { crate::src_devhost_service::ApplyDevicesPowerState(device, state) } != crate::types::HDF_SUCCESS {
                ret = crate::types::HDF_FAILURE;
            }
            node = unsafe { (*device).node.prev };
        }
    } else {
        let mut node = unsafe { (*host_service_ptr).devices.next };
        while !node.is_null() && node != &(*host_service_ptr).devices as *const crate::types::DListHead as *mut crate::types::DListHead {
            device = unsafe {
                (node as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize))
                    as *mut crate::types::HdfDevice
            };
            if unsafe { crate::src_devhost_service::ApplyDevicesPowerState(device, state) } != crate::types::HDF_SUCCESS {
                ret = crate::types::HDF_FAILURE;
            }
            node = unsafe { (*device).node.next };
        }
    }
    ret
}