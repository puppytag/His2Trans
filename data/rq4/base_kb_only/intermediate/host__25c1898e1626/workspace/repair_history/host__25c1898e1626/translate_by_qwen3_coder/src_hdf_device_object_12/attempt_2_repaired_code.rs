pub extern "C" fn HdfDeviceObjectAlloc(parent: *mut crate::types::HdfDeviceObject, driverName: *const ::core::ffi::c_char) -> *mut crate::types::HdfDeviceObject {
    use crate::types::*;
    use crate::compat::*;
    
    if parent.is_null() {
        unsafe {
            HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD002510u32,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to alloc device, parent invalid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return std::ptr::null_mut();
    }
    
    let offset = unsafe {
        &(*(std::ptr::null::<HdfDeviceNode>())).deviceObject as *const _ as usize
    };
    
    let parentDevNode = unsafe {
        (parent as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    if unsafe { (*parentDevNode).devStatus } != DEVNODE_LAUNCHED as u8 {
        unsafe {
            HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD002510u32,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to alloc device, parent status invalid %u\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return std::ptr::null_mut();
    }
    
    let newNode = unsafe {
        HdfObjectManagerGetObject(HDF_OBJECT_ID_DEVICE_SERVICE as i32) as *mut HdfDeviceNode
    };
    
    if newNode.is_null() {
        return std::ptr::null_mut();
    }
    
    let driver_name_copy = unsafe { HdfStringCopy(driverName) };
    if driver_name_copy.is_null() {
        crate::src_hdf_device_node::HdfDeviceNodeFreeInstance(newNode);
        return std::ptr::null_mut();
    }
    
    unsafe {
        (*newNode).driverName = driver_name_copy;
        (*newNode).hostService = (*parentDevNode).hostService;
        (*newNode).device = (*parentDevNode).device;
        
        &mut (*newNode).deviceObject as *mut HdfDeviceObject
    }
}