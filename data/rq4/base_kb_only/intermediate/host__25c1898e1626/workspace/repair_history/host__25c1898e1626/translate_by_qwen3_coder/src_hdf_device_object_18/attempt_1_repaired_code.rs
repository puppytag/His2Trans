pub extern "C" fn HdfDeviceObjectSetServInfo(dev: *mut crate::types::HdfDeviceObject, info: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    use crate::types::*;
    
    const SERVICE_INFO_LEN_MAX: usize = 128;
    
    if dev.is_null() || info.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    let info_len = unsafe { libc::strlen(info) };
    if info_len > SERVICE_INFO_LEN_MAX {
        return HDF_ERR_INVALID_PARAM;
    }
    
    // CONTAINER_OF: devNode = (struct HdfDeviceNode *)((char *)(dev) - (char *)&((struct HdfDeviceNode *)0)->deviceObject)
    // Use offset_of pattern without dereferencing null pointer
    let offset = std::mem::offset_of!(HdfDeviceNode, deviceObject);
    let devNode = unsafe { (dev as *mut u8).sub(offset) as *mut HdfDeviceNode };
    
    unsafe {
        if !(*devNode).servInfo.is_null() {
            crate::compat::OsalMemFree((*devNode).servInfo as *mut ::core::ffi::c_void);
        }
        (*devNode).servInfo = crate::compat::HdfStringCopy(info);
        if (*devNode).servInfo.is_null() {
            return HDF_ERR_MALLOC_FAIL;
        }
    }
    
    HDF_SUCCESS
}