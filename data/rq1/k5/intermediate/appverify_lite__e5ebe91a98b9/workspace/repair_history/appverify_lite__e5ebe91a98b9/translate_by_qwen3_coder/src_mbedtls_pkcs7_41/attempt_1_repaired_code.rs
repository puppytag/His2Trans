fn GetSignerDigestAlg(signer: *const crate::types::SignerInfo, algType: *mut crate::types::mbedtls_md_type_t) -> i32 {
    unsafe {
        let alg = &(*signer).digestAlgId;
        if !((crate::types::MBEDTLS_OID_DIGEST_ALG_SHA256.len() - 1) != (alg).len as usize || libc::memcmp(crate::types::MBEDTLS_OID_DIGEST_ALG_SHA256.as_ptr() as *const _, (alg).p as *const _, (alg).len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA256;
            return crate::types::PKCS7_SUCC as i32;
        }
        if !((crate::types::MBEDTLS_OID_DIGEST_ALG_SHA384.len() - 1) != (alg).len as usize || libc::memcmp(crate::types::MBEDTLS_OID_DIGEST_ALG_SHA384.as_ptr() as *const _, (alg).p as *const _, (alg).len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA384;
            return crate::types::PKCS7_SUCC as i32;
        }
        if !((crate::types::MBEDTLS_OID_DIGEST_ALG_SHA512.len() - 1) != (alg).len as usize || libc::memcmp(crate::types::MBEDTLS_OID_DIGEST_ALG_SHA512.as_ptr() as *const _, (alg).p as *const _, (alg).len as usize) != 0) {
            *algType = crate::types::MBEDTLS_MD_SHA512;
            return crate::types::PKCS7_SUCC as i32;
        }
        return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
    }
}