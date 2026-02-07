fn GetAppSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const _) };
    if sri.is_null() || unsafe { (*sri).nrOfSigners } == 0 {
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: Get all signer's resolved info failed\0".as_ptr() as *const i8,
            b"GetAppSingerCertType\0".as_ptr() as *const i8,
            456,
        ) };
        return crate::types::V_ERR as i32;
    }
    let ret = unsafe {
        crate::src_app_verify::GetAppCertTypeBySignInfo(
            (*sri).signers as *const crate::types::SignerResovledInfo,
            certType,
        )
    };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: get cert type by sign info failed: %d\0".as_ptr() as *const i8,
            b"GetAppSingerCertType\0".as_ptr() as *const i8,
            461,
            ret,
        ) };
        unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
        return crate::types::V_ERR as i32;
    }
    unsafe { crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri) };
    crate::types::V_OK as i32
}