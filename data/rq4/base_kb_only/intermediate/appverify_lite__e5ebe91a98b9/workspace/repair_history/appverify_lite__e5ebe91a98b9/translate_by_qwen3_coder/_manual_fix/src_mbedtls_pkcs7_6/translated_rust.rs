fn ParseSignerDigestAlg(p: *mut *mut u8, end: *const u8, signer: *mut crate::types::SignerInfo) -> i32 {
    unsafe {
        let rc = crate::compat::mbedtls_asn1_get_alg_null(
            p,
            end,
            &mut (*signer).digestAlgId,
        );
        if rc != 0 {
            return rc;
        }
        let alg_ptr: *const crate::types::mbedtls_asn1_buf = &(*signer).digestAlgId;
        if crate::src_mbedtls_pkcs7::InvalidDigestAlg(alg_ptr) {
            return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
        }
        crate::types::PKCS7_SUCC as i32
    }
}