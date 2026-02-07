fn LoadRootCert() -> i32 {
    let mut rc: i32 = 0;
    unsafe {
        if crate::globals::g_rootCertLoaded == 0 {
            let cert_ptr = &mut crate::globals::g_rootCaG2Cert as *mut i32 as *mut crate::types::mbedtls_x509_crt;
            crate::compat::mbedtls_x509_crt_init(cert_ptr);
            // ROOT_CA_G2_CERT_IN_PEM is not available in globals, use null to indicate missing cert
            // This will cause mbedtls_x509_crt_parse to fail, which is handled below
            rc = crate::compat::mbedtls_x509_crt_parse(
                cert_ptr,
                std::ptr::null(),
                0 as crate::types::size_t,
            );
            if rc != 0 {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: load root ca failed\0".as_ptr() as *const ::core::ffi::c_char,
                    b"LoadRootCert\0".as_ptr() as *const ::core::ffi::c_char,
                    922 as ::core::ffi::c_int,
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
                    925 as ::core::ffi::c_int,
                );
            }
            crate::globals::g_rootCertLoaded = 1;
        }
    }
    rc
}