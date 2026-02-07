pub extern "C" fn PKCS7_GetSignerSignningCertSubject(signer: *const crate::types::SignerInfo, subject: *mut ::core::ffi::c_char, subjectLen: crate::types::size_t) -> i32 {
    if signer.is_null() || subject.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    unsafe {
        let crt = (*signer).certPath.crt;
        if crt.is_null() {
            return crate::types::PKCS7_INVALID_PARAM as i32;
        }
        let rc = crate::compat::mbedtls_x509_dn_gets(subject, subjectLen, &(*crt).subject);
        if rc < 0 {
            return rc;
        }
        crate::types::PKCS7_SUCC as i32
    }
}