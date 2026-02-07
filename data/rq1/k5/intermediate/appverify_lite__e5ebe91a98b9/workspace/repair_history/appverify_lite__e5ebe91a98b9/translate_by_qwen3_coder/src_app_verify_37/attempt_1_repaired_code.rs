fn GetBinSignPkcs(signBuf: *const std::ffi::c_char, len: i32) -> *mut crate::types::Pkcs7 {
    let pkcs7 = unsafe { libc::malloc(std::mem::size_of::<crate::types::Pkcs7>()) } as *mut crate::types::Pkcs7;
    if pkcs7.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as i32, crate::types::LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: malloc error\0".as_ptr() as *const _, b"GetBinSignPkcs\0".as_ptr() as *const _, 1066) };
        return std::ptr::null_mut();
    }
    let ret = unsafe { crate::src_mbedtls_pkcs7::PKCS7_ParseSignedData(signBuf as *const ::core::ffi::c_uchar, len as crate::types::size_t, pkcs7) };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as i32, crate::types::LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: pkcs7parse message failed, ret: %d\0".as_ptr() as *const _, b"GetBinSignPkcs\0".as_ptr() as *const _, 1071, ret) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7) };
        if !pkcs7.is_null() {
            unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        }
        return std::ptr::null_mut();
    }
    pkcs7
}