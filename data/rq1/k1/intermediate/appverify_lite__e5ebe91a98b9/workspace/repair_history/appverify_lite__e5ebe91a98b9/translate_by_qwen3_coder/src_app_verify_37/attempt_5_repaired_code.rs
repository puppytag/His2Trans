fn GetBinSignPkcs(signBuf: *const std::ffi::c_char, len: i32) -> *mut crate::types::Pkcs7 {
    unsafe {
        let pkcs7 = libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) as *mut crate::types::Pkcs7;
        if pkcs7.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: malloc error\0".as_ptr() as *const std::ffi::c_char, __FUNCTION__!(), 1066);
            return std::ptr::null_mut();
        }
        let ret = crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(signBuf as *const ::core::ffi::c_uchar, len as crate::types::size_t, pkcs7);
        if ret != V_OK {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: pkcs7parse message failed, ret: %d\0".as_ptr() as *const std::ffi::c_char, __FUNCTION__!(), 1071, ret);
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            if !pkcs7.is_null() {
                libc::free(pkcs7 as *mut std::ffi::c_void);
            }
            return std::ptr::null_mut();
        }
        pkcs7
    }
}