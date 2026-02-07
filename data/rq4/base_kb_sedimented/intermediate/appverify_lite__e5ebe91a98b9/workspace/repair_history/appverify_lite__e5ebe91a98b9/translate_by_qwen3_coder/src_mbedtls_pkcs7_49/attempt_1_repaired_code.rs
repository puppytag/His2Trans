fn UnLoadSelfSignedCert() {
    unsafe {
        crate::compat::mbedtls_x509_crt_free(&mut crate::globals::g_ohosRootCert as *mut _ as *mut crate::types::mbedtls_x509_crt);
    }
}