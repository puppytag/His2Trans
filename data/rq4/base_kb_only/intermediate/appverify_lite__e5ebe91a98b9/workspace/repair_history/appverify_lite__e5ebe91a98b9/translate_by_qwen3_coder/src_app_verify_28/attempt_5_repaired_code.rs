pub extern "C" fn LoadCertAndCmpDest(certBase64: *const ::core::ffi::c_uchar, binSignCert: *const crate::types::CertInfo) -> i32 {
    if certBase64.is_null() || binSignCert.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let mut cert: crate::types::mbedtls_x509_crt = unsafe { std::mem::zeroed() };
    unsafe { crate::compat::mbedtls_x509_crt_init(&mut cert) };
    
    let cert_len = unsafe { libc::strlen(certBase64 as *const i8) } + 1;
    let ret = unsafe { crate::compat::mbedtls_x509_crt_parse(&mut cert, certBase64, cert_len as crate::types::size_t) };
    
    if ret != crate::types::V_OK as i32 {
        return crate::types::V_ERR as i32;
    }
    
    if CmpCert(&cert as *const _, binSignCert) == crate::types::V_OK as i32 {
        unsafe { crate::compat::mbedtls_x509_crt_free(&mut cert) };
        return crate::types::V_OK as i32;
    }
    
    unsafe { crate::compat::mbedtls_x509_crt_free(&mut cert) };
    crate::types::V_ERR as i32
}