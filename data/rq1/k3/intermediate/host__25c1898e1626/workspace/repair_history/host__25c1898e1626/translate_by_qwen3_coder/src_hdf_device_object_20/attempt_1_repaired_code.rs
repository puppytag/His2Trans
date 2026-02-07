pub extern "C" fn HdfDeviceObjectSetInterfaceDesc(dev: *mut crate::types::HdfDeviceObject, interfaceDesc: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if dev.is_null() || interfaceDesc.is_null() {
        return crate::types::HDF_ERR_INVALID_PARAM;
    }
    let dev_node_ptr = unsafe { (dev as *mut u8).offset(-(std::mem::offset_of!(crate::types::HdfDeviceNode, deviceObject) as isize)) as *mut crate::types::HdfDeviceNode };
    let copy = unsafe { crate::compat::HdfStringCopy(interfaceDesc) };
    unsafe {
        (*dev_node_ptr).interfaceDesc = copy;
    }
    if copy.is_null() {
        crate::types::HDF_ERR_MALLOC_FAIL
    } else {
        crate::types::HDF_SUCCESS
    }
}