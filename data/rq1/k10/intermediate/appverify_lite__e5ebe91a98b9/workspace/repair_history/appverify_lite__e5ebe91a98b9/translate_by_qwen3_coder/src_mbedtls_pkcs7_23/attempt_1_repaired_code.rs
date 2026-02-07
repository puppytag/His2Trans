fn DlogContentInfo(content: *const crate::types::Content) {
    if content.is_null() {
        return;
    }
    let len = unsafe { (*content).data.len };
    if len <= 0 {
        return;
    }
    let info = crate::src_mbedtls_pkcs7::Pkcs7Calloc((len + 1) as crate::types::size_t, std::mem::size_of::<std::ffi::c_char>() as crate::types::size_t);
    if info.is_null() {
        return;
    }
    let dest_max = (len + 1) as crate::types::size_t;
    let src = unsafe { (*content).data.p } as *const std::ffi::c_char;
    let ret = unsafe { crate::compat::strncpy_s(info as *mut std::ffi::c_char, dest_max, src, len as crate::types::size_t) };
    if ret != crate::types::EOK as i32 {
        crate::src_mbedtls_pkcs7::Pkcs7Free(info);
        return;
    }
    crate::src_mbedtls_pkcs7::Pkcs7Free(info);
}