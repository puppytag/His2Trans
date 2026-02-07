fn LoadRootCert() -> i32 {
    let mut rc: i32 = 0;
    unsafe {
        if !g_rootCertLoaded {
            mbedtls_x509_crt_init(&mut g_rootCaG2Cert);
            rc = mbedtls_x509_crt_parse(
                &mut g_rootCaG2Cert,
                ROOT_CA_G2_CERT_IN_PEM.as_ptr(),
                ROOT_CA_G2_CERT_IN_PEM.len() as crate::types::size_t,
            );
            if rc != 0 {
                let _ = HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: load root ca failed\0".as_ptr() as *const i8,
                    b"LoadRootCert\0".as_ptr() as *const i8,
                    922,
                );
                return rc;
            } else {
                let _ = HiLogPrint(
                    LOG_CORE,
                    LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: load root ca success\0".as_ptr() as *const i8,
                    b"LoadRootCert\0".as_ptr() as *const i8,
                    925,
                );
            }
            g_rootCertLoaded = true;
        }
    }
    rc
}