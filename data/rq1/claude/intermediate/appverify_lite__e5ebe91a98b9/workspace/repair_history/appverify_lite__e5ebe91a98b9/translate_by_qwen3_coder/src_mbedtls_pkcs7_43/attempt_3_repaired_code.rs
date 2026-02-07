pub extern "C" fn PKCS7_VerifySignerSignature(pkcs7: *const crate::types::Pkcs7, calcDigest: crate::types::PKCS7_CalcDigest) -> i32 {
    use crate::types::*;
    
    if pkcs7.is_null() || calcDigest.is_none() {
        return PKCS7_INVALID_PARAM as i32;
    }
    
    let mut rc: i32 = 0;
    let mut signer: *const SignerInfo = unsafe { &(*pkcs7).signedData.signers };
    let mut sig: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut sigLen: size_t = 0;
    
    while !signer.is_null() {
        rc = crate::src_mbedtls_pkcs7::GetSignerSignature(signer, &mut sig, &mut sigLen);
        if rc != 0 {
            return rc;
        }
        
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: get signer signature len : %zu\0".as_ptr() as *const ::core::ffi::c_char,
                b"PKCS7_VerifySignerSignature\0".as_ptr() as *const ::core::ffi::c_char,
                878i32,
                sigLen as usize,
            );
        }
        
        let mut pk: *mut mbedtls_pk_context = std::ptr::null_mut();
        rc = crate::src_mbedtls_pkcs7::GetSignerPubKeyOfSignature(signer, &mut pk);
        if rc != 0 {
            return rc;
        }
        
        let mut digAlg: mbedtls_md_type_t = 0;
        rc = crate::src_mbedtls_pkcs7::GetSignerDigestAlg(signer, &mut digAlg);
        if rc != 0 {
            return rc;
        }
        
        let mut hash: [::core::ffi::c_uchar; 64] = [0u8; 64];
        let mut hashLen: size_t = 0;
        
        let calc_fn = calcDigest.unwrap();
        rc = unsafe { calc_fn(pkcs7, signer, digAlg, hash.as_mut_ptr(), &mut hashLen) };
        if rc != 0 {
            unsafe {
                crate::compat::HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: Calculate content hash failed by calling callback\0".as_ptr() as *const ::core::ffi::c_char,
                    b"PKCS7_VerifySignerSignature\0".as_ptr() as *const ::core::ffi::c_char,
                    895i32,
                );
            }
            return rc;
        }
        
        let rsassa_pss_oid: &[u8] = b"\x2a\x86\x48\x86\xf7\x0d\x01\x01\x0a";
        let digest_enc_alg = unsafe { &(*signer).digestEncAlgId };
        let oid_len = rsassa_pss_oid.len();
        
        if (oid_len as u32) == digest_enc_alg.len {
            let cmp_result = unsafe {
                libc::memcmp(
                    rsassa_pss_oid.as_ptr() as *const ::core::ffi::c_void,
                    digest_enc_alg.p as *const ::core::ffi::c_void,
                    digest_enc_alg.len as usize,
                )
            };
            if cmp_result == 0 {
                unsafe {
                    crate::compat::mbedtls_rsa_set_padding(
                        (*pk).private_pk_ctx as *mut crate::types::mbedtls_rsa_context,
                        MBEDTLS_RSA_PKCS_V21 as i32,
                        0 as mbedtls_md_type_t,
                    );
                }
            }
        }
        
        rc = unsafe {
            crate::compat::mbedtls_pk_verify(
                pk,
                digAlg,
                hash.as_ptr(),
                hashLen as u32,
                sig,
                sigLen as u32,
            )
        };
        
        let _ = unsafe { crate::compat::memset_s(hash.as_mut_ptr() as *mut ::core::ffi::c_void, 64, 0, 64) };
        
        if rc != 0 {
            unsafe {
                crate::compat::HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: Verify signature failed, returned -0x%04x\0".as_ptr() as *const ::core::ffi::c_char,
                    b"PKCS7_VerifySignerSignature\0".as_ptr() as *const ::core::ffi::c_char,
                    905i32,
                    rc,
                );
            }
            return rc;
        } else {
            unsafe {
                crate::compat::HiLogPrint(
                    LOG_CORE,
                    LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: Verify signer signature success\n\0".as_ptr() as *const ::core::ffi::c_char,
                    b"PKCS7_VerifySignerSignature\0".as_ptr() as *const ::core::ffi::c_char,
                    908i32,
                );
            }
        }
        
        signer = unsafe { (*signer).next };
    }
    
    rc
}