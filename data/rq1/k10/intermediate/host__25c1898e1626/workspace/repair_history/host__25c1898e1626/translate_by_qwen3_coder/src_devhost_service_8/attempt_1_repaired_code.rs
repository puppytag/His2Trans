fn DevHostServicePmNotify(service: *mut crate::types::IDevHostService, state: u32) -> i32 {
    let mut device: *mut crate::types::HdfDevice = std::ptr::null_mut();
    let mut ret: i32 = crate::types::HDF_SUCCESS;
    let host_service_ptr = service as *mut u8;
    let super_offset = {
        let dummy: *mut crate::types::DevHostService = std::ptr::null_mut();
        let super_ptr = unsafe { &(*dummy).super_ } as *const crate::types::IDevHostService as *const u8;
        super_ptr as usize
    };
    let host_service = unsafe { host_service_ptr.offset(-(super_offset as isize)) as *mut crate::types::DevHostService };
    if host_service.is_null() {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD002510,
            "devhost_service\0".as_ptr() as *const i8,
            "failed to start device service, hostService is null\0".as_ptr() as *const i8,
        );
        return crate::types::HDF_FAILURE;
    }
    let host_name = unsafe { (*host_service).hostName };
    let host_name_str = if host_name.is_null() {
        "(null)"
    } else {
        unsafe { std::ffi::CStr::from_ptr(host_name).to_str().unwrap_or("(invalid)") }
    };
    let _ = crate::compat::HiLogPrint(
        crate::types::LOG_CORE,
        crate::types::LOG_DEBUG,
        0xD002510,
        "devhost_service\0".as_ptr() as *const i8,
        "host(%{public}s) set power state=%{public}u\0".as_ptr() as *const i8,
        host_name_str.as_ptr() as *const i8,
        state,
    );
    let is_wake = state == crate::types::POWER_STATE_DOZE_RESUME || state == crate::types::POWER_STATE_RESUME;
    if is_wake {
        let devices_head = unsafe { &(*host_service).devices };
        let devices_head_ptr = devices_head as *const crate::types::DListHead as *mut crate::types::DListHead;
        let mut node = unsafe { (*devices_head).prev };
        while node != devices_head_ptr {
            device = unsafe { (node as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize)) as *mut crate::types::HdfDevice };
            if crate::src_devhost_service::ApplyDevicesPowerState(device, state) != crate::types::HDF_SUCCESS {
                ret = crate::types::HDF_FAILURE;
            }
            node = unsafe { (*node).prev };
        }
    } else {
        let devices_head = unsafe { &(*host_service).devices };
        let devices_head_ptr = devices_head as *const crate::types::DListHead as *mut crate::types::DListHead;
        let mut node = unsafe { (*devices_head).next };
        while node != devices_head_ptr {
            device = unsafe { (node as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDevice, node) as isize)) as *mut crate::types::HdfDevice };
            if crate::src_devhost_service::ApplyDevicesPowerState(device, state) != crate::types::HDF_SUCCESS {
                ret = crate::types::HDF_FAILURE;
            }
            node = unsafe { (*node).next };
        }
    }
    ret
}