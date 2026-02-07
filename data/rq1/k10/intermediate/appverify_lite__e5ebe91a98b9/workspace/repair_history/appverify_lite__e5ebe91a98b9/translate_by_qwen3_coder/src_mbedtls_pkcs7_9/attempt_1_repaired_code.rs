fn ParseSignerEncAlg(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut params = crate::types::mbedtls_asn1_buf {
        tag: 0,
        len: 0,
        p: std::ptr::null_mut(),
    };
    unsafe {
        rc = crate::compat::mbedtls_asn1_get_alg(p, end, &mut (*signer).digestEncAlgId, &mut params);
    }
    if rc != 0 {
        return rc;
    }
    unsafe {
        if crate::src_mbedtls_pkcs7::InvalidDigestEncAlg(&(*signer).digestEncAlgId as *const crate::types::mbedtls_x509_buf) {
            return crate::types::PKCS7_INVALID_SIGNING_ALG as i32;
        }
    }
    return crate::types::PKCS7_SUCC as i32;
}