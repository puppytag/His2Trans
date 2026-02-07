fn UnLoadRootCert() {
    unsafe {
        if crate::globals::g_rootCertLoaded != 0 {
            crate::compat::mbedtls_x509_crt_free(
                std::ptr::addr_of_mut!(crate::globals::g_rootCaG2Cert)
            );
            crate::globals::g_rootCertLoaded = 0;
        }
    }
}