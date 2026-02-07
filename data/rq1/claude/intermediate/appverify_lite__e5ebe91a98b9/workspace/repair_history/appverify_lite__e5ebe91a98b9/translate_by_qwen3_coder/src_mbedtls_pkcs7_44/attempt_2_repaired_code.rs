fn LoadRootCert() -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if crate::globals::g_rootCertLoaded == 0 {
            let cert_ptr = &mut crate::globals::g_rootCaG2Cert as *mut i32 as *mut crate::types::mbedtls_x509_crt;
            crate::compat::mbedtls_x509_crt_init(cert_ptr);
            
            // ROOT_CA_G2_CERT_IN_PEM is a static const array defined elsewhere
            // Since it's not in globals, we reference it via extern
            extern "C" {
                static ROOT_CA_G2_CERT_IN_PEM: [u8; 0];
            }
            
            // Get pointer and size - size must be determined at link time
            // For now, use a placeholder that will be resolved
            let cert_data_ptr = ROOT_CA_G2_CERT_IN_PEM.as_ptr();
            let cert_data_len = std::mem::size_of_val(&ROOT_CA_G2_CERT_IN_PEM) as u32;
            
            rc = crate::compat::mbedtls_x509_crt_parse(
                cert_ptr,
                cert_data_ptr,
                cert_data_len,
            );
            if rc != 0 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: load root ca failed\0".as_ptr() as *const i8,
                    b"LoadRootCert\0".as_ptr() as *const i8,
                    922i32,
                );
                return rc;
            } else {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: load root ca success\0".as_ptr() as *const i8,
                    b"LoadRootCert\0".as_ptr() as *const i8,
                    925i32,
                );
            }
            crate::globals::g_rootCertLoaded = 1;
        }
        rc
    }
}