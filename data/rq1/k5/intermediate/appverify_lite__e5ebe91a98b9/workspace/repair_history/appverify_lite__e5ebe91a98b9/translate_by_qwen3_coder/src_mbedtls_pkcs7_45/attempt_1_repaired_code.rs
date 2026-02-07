fn UnLoadRootCert() {
    unsafe {
        if g_rootCertLoaded {
            mbedtls_x509_crt_free(&mut g_rootCaG2Cert);
            g_rootCertLoaded = false;
        }
    }
}