pub extern "C" fn PKCS7_VerifySignerSignature(pkcs7: *const crate::types::Pkcs7, calcDigest: crate::types::PKCS7_CalcDigest) -> i32 {
    let mut rc: i32 = 0;
    if pkcs7.is_null() || calcDigest.is_none() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    let mut signer: *const crate::types::SignerInfo = unsafe { &(*pkcs7).signedData.signers } as *const crate::types::SignerInfo;
    let mut sig: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut sigLen: crate::types::size_t = 0;
    while !signer.is_null() {
        rc = crate::src_mbedtls_pkcs7::GetSignerSignature(signer, &mut sig, &mut sigLen);
        if rc != 0 {
            return rc;
        }
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: get signer signature len : %zu\0".as_ptr() as *const ::core::ffi::c_char,
            b"PKCS7_VerifySignerSignature\0".as_ptr() as *const ::core::ffi::c_char,
            878,
            sigLen,
        ) };
        let mut pk: *mut crate::types::mbedtls_pk_context = std::ptr::null_mut();
        rc = crate::src_mbedtls_pkcs7::GetSignerPubKeyOfSignature(signer, &mut pk);
        if rc != 0 {
            return rc;
        }
        let mut digAlg: crate::types::mbedtls_md_type_t = 0;
        rc = crate::src_mbedtls_pkcs7::GetSignerDigestAlg(signer, &mut digAlg);
        if rc != 0 {
            return rc;
        }
        let mut hash: [::core::ffi::c_uchar; 64] = [0; 64];
        let _ = unsafe { crate::compat::memset_s(hash.as_mut_ptr() as *mut ::core::ffi::c_void, 64, 0, 64) };
        let mut hashLen: crate::types::size_t = 0;
        rc = if let Some(f) = calcDigest {
            unsafe { f(pkcs7, signer, digAlg, hash.as_mut_ptr(), &mut hashLen) }
        } else {
            return crate::types::PKCS7_INVALID_PARAM as i32;
        };
        if rc != 0 {
            let _ = unsafe { crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Calculate content hash failed by calling callback\0".as_ptr() as *const ::core::ffi::c_char,
                b"PKCS7_VerifySignerSignature\0".as_ptr() as *const ::core::ffi::c_char,
                895,
            ) };
            return rc;
        }
        unsafe {
            let digest_enc_alg = &(*signer).digestEncAlgId;
            let oid_rsassa_pss = crate::types::MBEDTLS_OID_RSASSA_PSS;
            let oid_len = (oid_rsassa_pss.len() - 1) as crate::types::size_t;
            if !(oid_len != digest_enc_alg.len || crate::compat::memcmp(
                oid_rsassa_pss.as_ptr() as *const ::core::ffi::c_void,
                digest_enc_alg.p as *const ::core::ffi::c_void,
                digest_enc_alg.len as usize,
            ) != 0) {
                let _ = crate::compat::mbedtls_rsa_set_padding((*pk).private_pk_ctx as *mut crate::compat::mbedtls_rsa_context, crate::types::MBEDTLS_RSA_PKCS_V21 as i32, 0);
            }
        }
        rc = unsafe { crate::compat::mbedtls_pk_verify(pk, digAlg, hash.as_ptr(), hashLen as u32, sig, sigLen as u32) };
        let _ = unsafe { crate::compat::memset_s(hash.as_mut_ptr() as *mut ::core::ffi::c_void, 64, 0, 64) };
        if rc != 0 {
            let _ = unsafe { crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Verify signature failed, returned -0x%04x\0".as_ptr() as *const ::core::ffi::c_char,
                b"PKCS7_VerifySignerSignature\0".as_ptr() as *const ::core::ffi::c_char,
                905,
                rc,
            ) };
            return rc;
        } else {
            let _ = unsafe { crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Verify signer signature success\n\0".as_ptr() as *const ::core::ffi::c_char,
                b"PKCS7_VerifySignerSignature\0".as_ptr() as *const ::core::ffi::c_char,
                908,
            ) };
        }
        signer = unsafe { (*signer).next } as *const crate::types::SignerInfo;
    }
    rc
}