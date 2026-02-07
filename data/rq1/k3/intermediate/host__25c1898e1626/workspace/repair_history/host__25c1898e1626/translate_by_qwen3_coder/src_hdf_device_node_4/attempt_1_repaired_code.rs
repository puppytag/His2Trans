函数: src_hdf_device_node_4
文件: src_hdf_device_node
尝试次数: 1/5
============================================================
修复后的代码:
============================================================
pub extern "C" fn HdfDeviceLaunchNode(devNode: *mut crate::types::HdfDeviceNode) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;
    if devNode.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"device_node\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to launch service, device or service is null\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let serv_name_ptr = unsafe { (*devNode).servName };
    let serv_name_str = if serv_name_ptr.is_null() {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        serv_name_ptr as *const ::core::ffi::c_char
    };
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_INFO as u32,
            0xD002510,
            b"device_node\0".as_ptr() as *const ::core::ffi::c_char,
            b"launch devnode %{public}s\0".as_ptr() as *const ::core::ffi::c_char,
            serv_name_str,
        );
    }
    let driver_ptr = unsafe { (*devNode).driver };
    if driver_ptr.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"device_node\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to launch service, deviceEntry invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let driver_entry = unsafe { (*driver_ptr).entry };
    if driver_entry.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"device_node\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to launch service, deviceEntry invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let init_fn = unsafe { (*driver_entry).Init };
    if init_fn.is_none() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_ERROR as u32,
                0xD002510,
                b"device_node\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to launch service, deviceEntry invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    unsafe {
        (*devNode).devStatus = crate::types::DEVNODE_LAUNCHED as u8;
    }
    ret = crate::src_hdf_device_node::DeviceDriverBind(devNode);
    if ret != crate::types::HDF_SUCCESS {
        return ret;
    }
    let device_object_ptr = unsafe { &mut (*devNode).deviceObject as *mut crate::types::HdfDeviceObject };
    ret = unsafe { init_fn.unwrap()(device_object_ptr) };
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_DEV_ERR_DEV_INIT_FAIL;
    }
    ret = crate::src_hdf_device_node::HdfDeviceNodePublishService(devNode);
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_DEV_ERR_PUBLISH_FAIL;
    }
    let token_ptr = unsafe { (*devNode).token };
    ret = crate::src_devmgr_service_clnt::DevmgrServiceClntAttachDevice(token_ptr);
    if ret != crate::types::HDF_SUCCESS {
        return crate::types::HDF_DEV_ERR_ATTACHDEV_FAIL;
    }
    ret
}