fn ParseSignerDigestAlg(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    let rc = unsafe { crate::compat::mbedtls_asn1_get_alg_null(p, end, &mut (*signer).digestAlgId) };
    if rc != 0 {
        return rc;
    }
    if unsafe { crate::src_mbedtls_pkcs7::InvalidDigestAlg(&(*signer).digestAlgId as *const _) } {
        return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
    }
    crate::types::PKCS7_SUCC as i32
}