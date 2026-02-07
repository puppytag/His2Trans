fn ParseSignedDataSignerInfos(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signers: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    unsafe {
        rc = mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x11);
        if rc != 0 || len == 0 {
            return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
        }
        let mut end_local = (*p).wrapping_add(len as usize);
        while *p < end_local {
            let mut one_signer_len: crate::types::size_t = 0;
            rc = mbedtls_asn1_get_tag(p, end_local, &mut one_signer_len, 0x20 | 0x10);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    387,
                    __FUNCTION__!(),
                    387,
                    rc,
                );
                return rc;
            }
            let one_signer_end = (*p).wrapping_add(one_signer_len as usize);
            rc = crate::src_mbedtls_pkcs7::ParseSignerVersion(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    392,
                    __FUNCTION__!(),
                    392,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerIssuerAndSerialNum(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    396,
                    __FUNCTION__!(),
                    396,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerDigestAlg(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    400,
                    __FUNCTION__!(),
                    400,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerAuthAttr(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    404,
                    __FUNCTION__!(),
                    404,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerEncAlg(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    408,
                    __FUNCTION__!(),
                    408,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerSignature(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    412,
                    __FUNCTION__!(),
                    412,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerUnAuthAttr(p, one_signer_end, signers);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    "appverify\0".as_ptr() as *const i8,
                    "[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const i8,
                    __FUNCTION__!(),
                    416,
                    __FUNCTION__!(),
                    416,
                    rc,
                );
                return rc;
            }
            if *p < end_local {
                let next = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<crate::types::SignerInfo>() as crate::types::size_t);
                if next.is_null() {
                    return crate::types::PKCS7_MEMORY_EXHAUST as i32;
                }
                (*signers).next = next as *mut crate::types::SignerInfo;
                signers = (*signers).next;
            }
        }
    }
    rc
}