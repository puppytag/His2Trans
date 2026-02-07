fn ParseSignedDataSignerInfos(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signers: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    unsafe {
        rc = crate::compat::mbedtls_asn1_get_tag(p, end, &mut len as *mut crate::types::size_t, 0x20 | 0x11);
    }
    if rc != 0 || len == 0 {
        return crate::types::PKCS7_HAS_NO_SIGNER_INFO as i32;
    }
    let end_local = unsafe { (*p).wrapping_add(len as usize) };
    let mut current_signers = signers;
    while unsafe { *p } < end_local {
        let mut one_signer_len: crate::types::size_t = 0;
        unsafe {
            rc = crate::compat::mbedtls_asn1_get_tag(p, end_local, &mut one_signer_len as *mut crate::types::size_t, 0x20 | 0x10);
        }
        if rc != crate::types::PKCS7_SUCC as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 387, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 387, rc) };
            return rc;
        }
        let one_signer_end = unsafe { (*p).wrapping_add(one_signer_len as usize) };
        rc = crate::src_mbedtls_pkcs7::ParseSignerVersion(p, one_signer_end, current_signers);
        if rc != crate::types::PKCS7_SUCC as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 392, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 392, rc) };
            return rc;
        }
        rc = crate::src_mbedtls_pkcs7::ParseSignerIssuerAndSerialNum(p, one_signer_end, current_signers);
        if rc != crate::types::PKCS7_SUCC as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 396, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 396, rc) };
            return rc;
        }
        rc = crate::src_mbedtls_pkcs7::ParseSignerDigestAlg(p, one_signer_end, current_signers);
        if rc != crate::types::PKCS7_SUCC as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 400, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 400, rc) };
            return rc;
        }
        rc = crate::src_mbedtls_pkcs7::ParseSignerAuthAttr(p, one_signer_end, current_signers);
        if rc != crate::types::PKCS7_SUCC as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 404, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 404, rc) };
            return rc;
        }
        rc = crate::src_mbedtls_pkcs7::ParseSignerEncAlg(p, one_signer_end, current_signers);
        if rc != crate::types::PKCS7_SUCC as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 408, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 408, rc) };
            return rc;
        }
        rc = crate::src_mbedtls_pkcs7::ParseSignerSignature(p, one_signer_end, current_signers);
        if rc != crate::types::PKCS7_SUCC as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 412, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 412, rc) };
            return rc;
        }
        rc = crate::src_mbedtls_pkcs7::ParseSignerUnAuthAttr(p, one_signer_end, current_signers);
        if rc != crate::types::PKCS7_SUCC as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: %s:%u, error occurred, ret:%d\0".as_ptr() as *const ::core::ffi::c_char, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 416, b"ParseSignedDataSignerInfos\0".as_ptr() as *const ::core::ffi::c_char, 416, rc) };
            return rc;
        }
        if unsafe { *p } < end_local {
            let next = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<crate::types::SignerInfo>() as crate::types::size_t);
            if next.is_null() {
                return crate::types::PKCS7_MEMORY_EXHAUST as i32;
            }
            unsafe {
                (*current_signers).next = next as *mut crate::types::SignerInfo;
                current_signers = (*current_signers).next;
            }
        }
    }
    rc
}