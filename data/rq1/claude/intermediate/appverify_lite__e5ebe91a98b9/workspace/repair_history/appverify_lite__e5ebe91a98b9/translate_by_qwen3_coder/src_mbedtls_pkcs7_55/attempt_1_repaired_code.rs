pub extern "C" fn PKCS7_GetSignerSignningCertSubject(signer: *const crate::types::SignerInfo, subject: *mut ::core::ffi::c_char, subjectLen: crate::types::size_t) -> i32 {
    if signer.is_null() || subject.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    
    let crt: *const crate::types::mbedtls_x509_crt = unsafe { (*signer).certPath.crt };
    
    let rc = unsafe {
        crate::compat::mbedtls_x509_dn_gets(
            subject,
            subjectLen,
            &(*crt).subject as *const crate::types::mbedtls_x509_name
        )
    };
    
    if rc < 0 {
        return rc;
    }
    
    crate::types::PKCS7_SUCC as i32
}