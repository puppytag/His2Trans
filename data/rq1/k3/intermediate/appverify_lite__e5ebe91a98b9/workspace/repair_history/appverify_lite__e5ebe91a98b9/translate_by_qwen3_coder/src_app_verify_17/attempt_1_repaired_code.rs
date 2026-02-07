fn GetProfileSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const crate::types::Pkcs7);
    if sri.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: Get all signer's resolved info failed\0".as_ptr() as *const i8,
                b"GetProfileSingerCertType\0".as_ptr() as *const i8,
                474,
            );
        }
        return crate::types::V_ERR as i32;
    }
    let signer_ptr = unsafe { (*sri).signers };
    let ret = crate::src_app_verify::GetProfileCertTypeBySignInfo(signer_ptr, certType);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: get cert type by sign info failed: %d\0".as_ptr() as *const i8,
                b"GetProfileSingerCertType\0".as_ptr() as *const i8,
                479,
                ret,
            );
        }
        crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
        return crate::types::V_ERR as i32;
    }
    crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
    crate::types::V_OK as i32
}