fn ParseSignerEncAlg(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut params: crate::types::mbedtls_asn1_buf = crate::types::mbedtls_asn1_buf {
        tag: 0,
        len: 0,
        p: std::ptr::null_mut(),
    };

    let rc = unsafe {
        crate::compat::mbedtls_asn1_get_alg(
            p,
            end,
            &mut (*signer).digestEncAlgId as *mut crate::types::mbedtls_x509_buf,
            &mut params,
        )
    };
    if rc != 0 {
        return rc;
    }
    let alg_ptr: *const crate::types::mbedtls_x509_buf = unsafe { &(*signer).digestEncAlgId };
    let invalid = InvalidDigestEncAlg(alg_ptr);
    if invalid {
        return crate::types::PKCS7_INVALID_SIGNING_ALG as i32;
    }
    crate::types::PKCS7_SUCC as i32
}