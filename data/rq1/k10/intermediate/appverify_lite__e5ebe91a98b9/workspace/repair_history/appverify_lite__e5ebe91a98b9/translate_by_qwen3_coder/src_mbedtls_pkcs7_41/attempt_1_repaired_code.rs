fn GetSignerDigestAlg(signer: *const crate::types::SignerInfo, algType: *mut crate::types::mbedtls_md_type_t) -> i32 {
    unsafe {
        let alg = &(*signer).digestAlgId;
        let sha256_oid = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA256;
        let sha384_oid = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA384;
        let sha512_oid = crate::types::MBEDTLS_OID_DIGEST_ALG_SHA512;
        if !((sha256_oid.len() - 1) != alg.len as usize || libc::memcmp(sha256_oid.as_ptr() as *const _, alg.p as *const _, alg.len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA256;
            return crate::types::PKCS7_SUCC as i32;
        }
        if !((sha384_oid.len() - 1) != alg.len as usize || libc::memcmp(sha384_oid.as_ptr() as *const _, alg.p as *const _, alg.len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA384;
            return crate::types::PKCS7_SUCC as i32;
        }
        if !((sha512_oid.len() - 1) != alg.len as usize || libc::memcmp(sha512_oid.as_ptr() as *const _, alg.p as *const _, alg.len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA512;
            return crate::types::PKCS7_SUCC as i32;
        }
        return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
    }
}