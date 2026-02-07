pub extern "C" fn PKCS7_GetSignerSignningCertIssuer(signer: *const crate::types::SignerInfo, issuer: *mut ::core::ffi::c_char, issuerLen: crate::types::size_t) -> i32 {
    use crate::types::{PKCS7_INVALID_PARAM, PKCS7_SUCC};
    
    if signer.is_null() || issuer.is_null() {
        return PKCS7_INVALID_PARAM as i32;
    }
    
    unsafe {
        let crt: *const crate::types::mbedtls_x509_crt = (*signer).certPath.crt;
        let rc = crate::compat::mbedtls_x509_dn_gets(
            issuer,
            issuerLen as u32,
            &(*crt).issuer as *const crate::types::mbedtls_x509_name
        );
        if rc < 0 {
            return rc;
        }
        PKCS7_SUCC as i32
    }
}