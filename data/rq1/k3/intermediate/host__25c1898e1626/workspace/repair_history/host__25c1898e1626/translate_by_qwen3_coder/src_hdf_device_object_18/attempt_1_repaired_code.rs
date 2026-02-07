pub extern "C" fn HdfDeviceObjectSetServInfo(dev: *mut crate::types::HdfDeviceObject, info: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if dev.is_null() || info.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let len = unsafe { libc::strlen(info) };
    if len > 128 {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_node_ptr = unsafe { (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) } as *mut crate::types::HdfDeviceNode;
    let dev_node = unsafe { &mut *dev_node_ptr };
    if !dev_node.servInfo.is_null() {
        unsafe {
            OsalMemFree(dev_node.servInfo as *mut ::core::ffi::c_void);
        }
    }
    unsafe {
        dev_node.servInfo = HdfStringCopy(info);
    }
    if dev_node.servInfo.is_null() {
        return crate::types::HDF_ERR_MALLOC_FAIL;
    }
    crate::types::HDF_SUCCESS
}