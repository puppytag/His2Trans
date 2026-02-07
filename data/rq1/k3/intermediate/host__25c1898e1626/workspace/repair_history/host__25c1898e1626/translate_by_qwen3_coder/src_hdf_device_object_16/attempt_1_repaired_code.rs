pub extern "C" fn HdfDeviceObjectPublishService(dev: *mut crate::types::HdfDeviceObject, servName: *const ::core::ffi::c_char, policy: u8, perm: u32) -> ::core::ffi::c_int {
    use crate::types::*;
    if dev.is_null() || servName.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    if policy <= SERVICE_POLICY_NONE as u8 || policy >= SERVICE_POLICY_INVALID as u8 {
        return HDF_DEV_ERR_NO_DEVICE_SERVICE;
    }
    let dev_node_ptr = unsafe { (dev as *mut u8).offset(-(std::mem::offset_of!(HdfDeviceNode, deviceObject) as isize)) as *mut HdfDeviceNode };
    let dev_node = unsafe { &mut *dev_node_ptr };
    if dev_node.servStatus {
        let _ = unsafe { crate::compat::HiLogPrint(LOG_CORE as u32, LOG_ERROR as u32, 0xD002510, b"device_object\0".as_ptr() as *const _, b"failed to publish public service, repeat publish\0".as_ptr() as *const _) };
        return HDF_FAILURE;
    }
    dev_node.servName = unsafe { crate::compat::HdfStringCopy(servName) };
    if dev_node.servName.is_null() {
        return HDF_DEV_ERR_NO_MEMORY;
    }
    dev_node.policy = policy as u16;
    dev_node.permission = perm as u16;
    let ret = crate::src_hdf_device_node::DeviceDriverBind(dev_node_ptr);
    if ret != HDF_SUCCESS {
        return ret;
    }
    if let Some(f) = dev_node.super_.PublishService {
        unsafe { f(dev_node_ptr) }
    } else {
        0
    }
}