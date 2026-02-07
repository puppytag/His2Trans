fn UnLoadSelfSignedCert() {
    unsafe {
        crate::compat::mbedtls_x509_crt_free(std::ptr::addr_of_mut!(crate::globals::g_ohosRootCert) as *mut crate::types::mbedtls_x509_crt);
    }
}