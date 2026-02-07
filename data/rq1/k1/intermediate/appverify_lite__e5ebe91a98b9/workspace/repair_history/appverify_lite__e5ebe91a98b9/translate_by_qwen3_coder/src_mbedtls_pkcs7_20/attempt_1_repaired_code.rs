fn ParseSignedDataSignerInfos(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signers: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    unsafe {
        rc = mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x11);
        if rc != 0 || len == 0 {
            return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
        }
        let end = (*p).wrapping_add(len as usize);
        let mut current_signer = signers;
        while *p < end {
            let mut one_signer_len: crate::types::size_t = 0;
            rc = mbedtls_asn1_get_tag(p, end, &mut one_signer_len, 0x20 | 0x10);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    387,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    387,
                    rc,
                );
                return rc;
            }
            let one_signer_end = (*p).wrapping_add(one_signer_len as usize);
            rc = crate::src_mbedtls_pkcs7::ParseSignerVersion(p, one_signer_end, current_signer);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    392,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    392,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerIssuerAndSerialNum(p, one_signer_end, current_signer);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    396,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    396,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerDigestAlg(p, one_signer_end, current_signer);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    400,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    400,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerAuthAttr(p, one_signer_end, current_signer);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    404,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    404,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerEncAlg(p, one_signer_end, current_signer);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    408,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    408,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerSignature(p, one_signer_end, current_signer);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    412,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    412,
                    rc,
                );
                return rc;
            }
            rc = crate::src_mbedtls_pkcs7::ParseSignerUnAuthAttr(p, one_signer_end, current_signer);
            if rc != crate::types::PKCS7_SUCC as i32 {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    416,
                    b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char,
                    416,
                    rc,
                );
                return rc;
            }
            if *p < end {
                let next = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<crate::types::SignerInfo>() as crate::types::size_t);
                if next.is_null() {
                    return crate::types::PKCS7_MEMORY_EXHAUST as i32;
                }
                (*current_signer).next = next as *mut crate::types::SignerInfo;
                current_signer = (*current_signer).next;
            }
        }
    }
    rc
}