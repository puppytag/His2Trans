pub extern "C" fn HdfDeviceObjectUnRegister(dev: *mut crate::types::HdfDeviceObject) -> ::core::ffi::c_int {
    use crate::types::{HdfDeviceNode, HdfDeviceObject, HDF_ERR_INVALID_OBJECT};
    
    // Calculate offset of deviceObject field within HdfDeviceNode using offset_of approach
    let offset = std::mem::offset_of!(HdfDeviceNode, deviceObject);
    
    let devNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    if devNode.is_null() {
        return HDF_ERR_INVALID_OBJECT;
    }
    
    let device = unsafe { (*devNode).device };
    if device.is_null() {
        return HDF_ERR_INVALID_OBJECT;
    }
    
    let detach_fn = unsafe { (*device).super_.Detach };
    if let Some(detach) = detach_fn {
        unsafe {
            detach(&mut (*device).super_ as *mut crate::types::IHdfDevice, devNode)
        }
    } else {
        HDF_ERR_INVALID_OBJECT
    }
}