pub extern "C" fn PKCS7_GetSignerSignningCertIssuer(signer: *const crate::types::SignerInfo, issuer: *mut ::core::ffi::c_char, issuerLen: crate::types::size_t) -> i32 {
    if signer.is_null() || issuer.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    let crt = unsafe { (*signer).certPath.crt };
    if crt.is_null() {
        return 0;
    }
    let rc = unsafe { crate::compat::mbedtls_x509_dn_gets(issuer, issuerLen, &(*crt).issuer) };
    if rc < 0 {
        return rc;
    }
    crate::types::PKCS7_SUCC as i32
}