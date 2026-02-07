pub extern "C" fn HdfDeviceObjectSetServInfo(dev: *mut crate::types::HdfDeviceObject, info: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    use crate::types::*;
    use crate::compat::*;
    
    const SERVICE_INFO_LEN_MAX: usize = 128;
    
    if dev.is_null() || info.is_null() {
        return HDF_ERR_INVALID_PARAM;
    }
    
    let info_len = unsafe { libc::strlen(info) };
    if info_len > SERVICE_INFO_LEN_MAX {
        return HDF_ERR_INVALID_PARAM;
    }
    
    // Calculate offset using memoffset-style approach without null deref
    let offset = {
        let base: usize = 0;
        let field_ptr = unsafe {
            &(*((base as *const u8) as *const HdfDeviceNode)).deviceObject as *const _ as usize
        };
        field_ptr - base
    };
    let devNode = unsafe {
        (dev as *mut u8).sub(offset) as *mut HdfDeviceNode
    };
    
    unsafe {
        if !(*devNode).servInfo.is_null() {
            OsalMemFree((*devNode).servInfo as *mut ::core::ffi::c_void);
        }
        (*devNode).servInfo = HdfStringCopy(info);
        if (*devNode).servInfo.is_null() {
            return HDF_ERR_MALLOC_FAIL;
        }
    }
    
    HDF_SUCCESS
}