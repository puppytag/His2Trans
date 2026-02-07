fn DevHostServicePmNotify(service: *mut crate::types::IDevHostService, state: u32) -> i32 {
    let mut device: *mut crate::types::HdfDevice = std::ptr::null_mut();
    let mut ret: i32 = crate::types::HDF_SUCCESS;
    let hostService: *mut crate::types::DevHostService = if !service.is_null() {
        unsafe {
            (service as *mut u8).offset(-(std::mem::offset_of!(crate::types::DevHostService, super_) as isize))
                as *mut crate::types::DevHostService
        }
    } else {
        std::ptr::null_mut()
    };
    if hostService.is_null() {
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
    let host_name_ptr = unsafe { (*hostService).hostName };
    let host_name = if host_name_ptr.is_null() {
        "(null)"
    } else {
        unsafe { std::ffi::CStr::from_ptr(host_name_ptr).to_str().unwrap_or("(invalid)") }
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
        let mut node_ptr = unsafe { (*hostService).devices.prev };
        while node_ptr != unsafe { &mut (*hostService).devices as *mut crate::types::DListHead } {
            device = unsafe {
                (node_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize))
                    as *mut crate::types::HdfDevice
            };
            if crate::src_devhost_service::ApplyDevicesPowerState(device, state) != crate::types::HDF_SUCCESS {
                ret = crate::types::HDF_FAILURE;
            }
            node_ptr = unsafe { (*device).node.prev };
        }
    } else {
        let mut node_ptr = unsafe { (*hostService).devices.next };
        while node_ptr != unsafe { &mut (*hostService).devices as *mut crate::types::DListHead } {
            device = unsafe {
                (node_ptr as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize))
                    as *mut crate::types::HdfDevice
            };
            if crate::src_devhost_service::ApplyDevicesPowerState(device, state) != crate::types::HDF_SUCCESS {
                ret = crate::types::HDF_FAILURE;
            }
            node_ptr = unsafe { (*device).node.next };
        }
    }
    ret
}