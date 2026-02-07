fn DlogContentInfo(content: *const crate::types::Content) {
    if content.is_null() {
        return;
    }
    
    let len: i32 = unsafe { (*content).data.len as i32 };
    if len <= 0 {
        return;
    }
    
    let info: *mut std::ffi::c_char = crate::src_mbedtls_pkcs7::Pkcs7Calloc(
        (len + 1) as crate::types::size_t,
        std::mem::size_of::<std::ffi::c_char>() as crate::types::size_t
    ) as *mut std::ffi::c_char;
    
    if info.is_null() {
        return;
    }
    
    let ret = unsafe {
        strncpy_s(
            info,
            (len + 1) as u32,
            (*content).data.p as *const std::ffi::c_char,
            len as u32
        )
    };
    
    if ret != crate::types::EOK as i32 {
        crate::src_mbedtls_pkcs7::Pkcs7Free(info as *mut std::ffi::c_void);
        return;
    }
    
    crate::src_mbedtls_pkcs7::Pkcs7Free(info as *mut std::ffi::c_void);
}