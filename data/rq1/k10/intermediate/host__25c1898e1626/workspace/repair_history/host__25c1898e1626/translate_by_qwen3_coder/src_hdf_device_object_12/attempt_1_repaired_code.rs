pub extern "C" fn HdfDeviceObjectAlloc(parent: *mut crate::types::HdfDeviceObject, driverName: *const ::core::ffi::c_char) -> *mut crate::types::HdfDeviceObject {
    let mut newNode: *mut crate::types::HdfDeviceNode = std::ptr::null_mut();
    let parentDevNode: *mut crate::types::HdfDeviceNode = if !parent.is_null() {
        unsafe {
            (parent as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode
        }
    } else {
        std::ptr::null_mut()
    };

    if parent.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as i32, crate::types::LOG_ERROR as i32, 0xD002510, b"device_object\0".as_ptr() as *const _, b"failed to alloc device, parent invalid\0".as_ptr() as *const _) };
        return std::ptr::null_mut();
    }

    unsafe {
        if (*parentDevNode).devStatus as u32 != crate::types::DEVNODE_LAUNCHED {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as i32, crate::types::LOG_ERROR as i32, 0xD002510, b"device_object\0".as_ptr() as *const _, b"failed to alloc device, parent status invalid %{public}u\0".as_ptr() as *const _, (*parentDevNode).devStatus as u32);
            return std::ptr::null_mut();
        }
    }

    newNode = unsafe { crate::compat::HdfObjectManagerGetObject(crate::types::HDF_OBJECT_ID_DEVICE_SERVICE as i32) as *mut crate::types::HdfDeviceNode };
    if newNode.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*newNode).driverName = crate::compat::HdfStringCopy(driverName);
        if (*newNode).driverName.is_null() {
            crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(newNode);
            return std::ptr::null_mut();
        }
        (*newNode).hostService = (*parentDevNode).hostService;
        (*newNode).device = (*parentDevNode).device;
        &mut (*newNode).deviceObject
    }
}