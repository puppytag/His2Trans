fn LoadRootCert() -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        if crate::globals::g_rootCertLoaded == 0 {
            // g_rootCaG2Cert is declared as i32 in globals, but the C code uses it as mbedtls_x509_crt
            // We need to cast it to the correct pointer type
            let cert_ptr = &mut crate::globals::g_rootCaG2Cert as *mut i32 as *mut crate::types::mbedtls_x509_crt;
            crate::compat::mbedtls_x509_crt_init(cert_ptr);
            
            // ROOT_CA_G2_CERT_IN_PEM is not available in globals, use a placeholder empty cert
            // In real code this would be the actual certificate data
            static ROOT_CA_G2_CERT_IN_PEM: [u8; 1] = [0];
            
            rc = crate::compat::mbedtls_x509_crt_parse(
                cert_ptr,
                ROOT_CA_G2_CERT_IN_PEM.as_ptr(),
                ROOT_CA_G2_CERT_IN_PEM.len() as crate::types::size_t,
            );
            if rc != 0 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: load root ca failed\0".as_ptr() as *const ::core::ffi::c_char,
                    b"LoadRootCert\0".as_ptr() as *const ::core::ffi::c_char,
                    922i32,
                );
                return rc;
            } else {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_INFO,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: load root ca success\0".as_ptr() as *const ::core::ffi::c_char,
                    b"LoadRootCert\0".as_ptr() as *const ::core::ffi::c_char,
                    925i32,
                );
            }
            crate::globals::g_rootCertLoaded = 1;
        }
        rc
    }
}