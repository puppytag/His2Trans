pub extern "C" fn HdfDeviceObjectPublishService(dev: *mut crate::types::HdfDeviceObject, servName: *const ::core::ffi::c_char, policy: u8, perm: u32) -> ::core::ffi::c_int {
    use crate::types::*;
    
    if dev.is_null() || servName.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    // CONTAINER_OF: devNode = (struct HdfDeviceNode *)((char *)(dev) - (char *)&((struct HdfDeviceNode *)0)->deviceObject)
    // Calculate offset using memoffset-style approach without null pointer dereference
    let devNode: *mut HdfDeviceNode = unsafe {
        let offset = core::mem::offset_of!(HdfDeviceNode, deviceObject);
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    if policy <= SERVICE_POLICY_NONE as u8 || policy >= SERVICE_POLICY_INVALID as u8 {
        return HDF_DEV_ERR_NO_DEVICE_SERVICE;
    }
    
    unsafe {
        if (*devNode).servStatus {
            let _ = crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD002510,
                b"device_object\0".as_ptr() as *const ::core::ffi::c_char,
                b"failed to publish public service, repeat publish\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return HDF_FAILURE;
        }
        
        (*devNode).servName = crate::compat::HdfStringCopy(servName);
        if (*devNode).servName.is_null() {
            return HDF_DEV_ERR_NO_MEMORY;
        }
        
        (*devNode).policy = policy as u16;
        (*devNode).permission = perm as u16;
        
        let ret = crate::src_hdf_device_node::DeviceDriverBind(devNode);
        if ret != HDF_SUCCESS {
            return ret;
        }
        
        if let Some(publish_fn) = (*devNode).super_.PublishService {
            publish_fn(devNode)
        } else {
            HDF_FAILURE
        }
    }
}