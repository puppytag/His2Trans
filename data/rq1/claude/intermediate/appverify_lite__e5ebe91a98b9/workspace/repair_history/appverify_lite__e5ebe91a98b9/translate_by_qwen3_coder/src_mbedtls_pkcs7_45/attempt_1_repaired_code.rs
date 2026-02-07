fn UnLoadRootCert() {
    unsafe {
        if crate::globals::g_rootCertLoaded != 0 {
            crate::compat::mbedtls_x509_crt_free(&mut crate::globals::g_rootCaG2Cert as *mut crate::types::mbedtls_x509_crt);
            crate::globals::g_rootCertLoaded = 0;
        }
    }
}