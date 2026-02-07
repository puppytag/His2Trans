pub extern "C" fn PKCS7_FreeAllSignersResolvedInfo(sri: *mut crate::types::SignersResovedInfo) {
    if sri.is_null() {
        return;
    }
    unsafe {
        if !(*sri).signers.is_null() {
            crate::src_mbedtls_pkcs7::Pkcs7Free((*sri).signers as *mut ::core::ffi::c_void);
            (*sri).signers = std::ptr::null_mut();
        }
        crate::src_mbedtls_pkcs7::Pkcs7Free(sri as *mut ::core::ffi::c_void);
    }
}