fn DevHostServiceQueryOrAddDevice(inst: *mut crate::types::DevHostService, deviceId: u16) -> *mut crate::types::HdfDevice {
    let mut device = crate::src_devhost_service::DevHostServiceFindDevice(inst, deviceId);
    if device.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"%{public}s can't find device, try to create\0".as_ptr() as *const _,
                b"DevHostServiceQueryOrAddDevice\0".as_ptr() as *const _,
            );
        }
        device = crate::src_hdf_device::HdfDeviceNewInstance();
        if device.is_null() {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD002510,
                    b"devhost_service\0".as_ptr() as *const _,
                    b"Dev host service failed to create driver instance\0".as_ptr() as *const _,
                );
            }
            return std::ptr::null_mut();
        }
        unsafe {
            let host_id = (*inst).hostId;
            (*device).deviceId = ((host_id as u32) << (16 + 8)) | ((deviceId as u32) << 8) | 0;
            let entry = &mut (*device).node;
            let head = &mut (*inst).devices;
            entry.next = head.next;
            entry.prev = head;
            (*head.next).prev = entry;
            head.next = entry;
        }
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD002510,
                b"devhost_service\0".as_ptr() as *const _,
                b"%{public}s add device complete\0".as_ptr() as *const _,
                b"DevHostServiceQueryOrAddDevice\0".as_ptr() as *const _,
            );
        }
    }
    device
}