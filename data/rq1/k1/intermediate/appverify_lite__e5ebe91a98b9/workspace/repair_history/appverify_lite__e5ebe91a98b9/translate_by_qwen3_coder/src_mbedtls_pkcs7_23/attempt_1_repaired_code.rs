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
    let src = unsafe { (*content).data.p } as *const std::ffi::c_char;
    let dest = info as *mut std::ffi::c_char;
    let dest_max = (len + 1) as crate::types::size_t;
    let count = len as crate::types::size_t;
    if unsafe { crate::compat::strncpy_s(dest, dest_max, src, count) } != 0 {
        crate::src_mbedtls_pkcs7::Pkcs7Free(info);
        return;
    }
    crate::src_mbedtls_pkcs7::Pkcs7Free(info);
}