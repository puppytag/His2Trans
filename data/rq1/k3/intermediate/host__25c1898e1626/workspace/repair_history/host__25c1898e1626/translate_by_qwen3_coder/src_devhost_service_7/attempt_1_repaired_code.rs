fn ApplyDevicesPowerState(device: *mut crate::types::HdfDevice, state: u32) -> i32 {
    let mut device_node: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let mut ret: i32;

    if state == crate::types::POWER_STATE_DOZE_RESUME || state == crate::types::POWER_STATE_RESUME {
        let mut current = unsafe { (*device).devNodes.next };
        while current != &mut unsafe { (*device).devNodes } as *mut crate::types::DListHead {
            device_node = unsafe {
                (current as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode
            };
            if unsafe { (*device_node).powerToken.is_null() } {
                current = unsafe { (*current).next };
                continue;
            }
            ret = crate::src_power_state_token::PowerStateChange(unsafe { (*device_node).powerToken }, state);
            if ret != crate::types::HDF_SUCCESS {
                let _ = unsafe {
                    crate::compat::HiLogPrint(
                        crate::types::LOG_CORE,
                        crate::types::LOG_ERROR,
                        0xD002510,
                        "devhost_service\0".as_ptr() as *const i8,
                        "device %{public}s failed to resume(%{public}u)\0".as_ptr() as *const i8,
                        if !unsafe { (*device_node).driver }.is_null()
                            && !unsafe { (*(*device_node).driver).entry }.is_null()
                        {
                            unsafe { (*(*(*device_node).driver).entry).moduleName }
                        } else {
                            std::ptr::null()
                        },
                        state,
                    )
                };
            }
            current = unsafe { (*current).next };
        }
    } else {
        let mut current = unsafe { (*device).devNodes.prev };
        while current != &mut unsafe { (*device).devNodes } as *mut crate::types::DListHead {
            device_node = unsafe {
                (current as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, entry) as isize)) as *mut crate::types::HdfDeviceNode
            };
            if unsafe { (*device_node).powerToken.is_null() } {
                current = unsafe { (*current).prev };
                continue;
            }
            ret = crate::src_power_state_token::PowerStateChange(unsafe { (*device_node).powerToken }, state);
            if ret != crate::types::HDF_SUCCESS {
                let _ = unsafe {
                    crate::compat::HiLogPrint(
                        crate::types::LOG_CORE,
                        crate::types::LOG_ERROR,
                        0xD002510,
                        "devhost_service\0".as_ptr() as *const i8,
                        "device %{public}s failed to suspend(%{public}u)\0".as_ptr() as *const i8,
                        if !unsafe { (*device_node).driver }.is_null()
                            && !unsafe { (*(*device_node).driver).entry }.is_null()
                        {
                            unsafe { (*(*(*device_node).driver).entry).moduleName }
                        } else {
                            std::ptr::null()
                        },
                        state,
                    )
                };
            }
            current = unsafe { (*current).prev };
        }
    }

    crate::types::HDF_SUCCESS
}