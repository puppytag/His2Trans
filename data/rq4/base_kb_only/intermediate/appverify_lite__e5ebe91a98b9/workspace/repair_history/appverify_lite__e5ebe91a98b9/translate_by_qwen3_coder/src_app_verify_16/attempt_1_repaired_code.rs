fn GetAppSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const crate::types::Pkcs7);
    if sri.is_null() || unsafe { (*sri).nrOfSigners } == 0 {
        crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Get all signer's resolved info failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetAppSingerCertType\0".as_ptr() as *const ::core::ffi::c_char,
                456i32,
            )
        };
        return crate::types::V_ERR as i32;
    }
    let ret = crate::src_app_verify::GetAppCertTypeBySignInfo(unsafe { (*sri).signers }, certType);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: get cert type by sign info failed: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"GetAppSingerCertType\0".as_ptr() as *const ::core::ffi::c_char,
                461i32,
                ret,
            )
        };
        crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
        return crate::types::V_ERR as i32;
    }
    crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
    crate::types::V_OK as i32
}