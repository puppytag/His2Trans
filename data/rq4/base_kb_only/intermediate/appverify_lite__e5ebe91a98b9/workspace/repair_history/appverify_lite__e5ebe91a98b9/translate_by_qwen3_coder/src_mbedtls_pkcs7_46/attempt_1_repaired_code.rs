fn LoadDebugModeRootCert() -> i32 {
    unsafe {
        crate::compat::mbedtls_x509_crt_init(&mut crate::globals::g_debugModeRootCert);
        let rc = crate::compat::mbedtls_x509_crt_parse(
            &mut crate::globals::g_debugModeRootCert,
            crate::globals::DEBUG_MODE_ROOT_CERT_IN_PEM!().as_ptr(),
            std::mem::size_of_val(&crate::globals::DEBUG_MODE_ROOT_CERT_IN_PEM!()) as usize,
        );
        if rc != 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: load debug mode root ca failed %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadDebugModeRootCert\0".as_ptr() as *const ::core::ffi::c_char,
                946i32,
                rc,
            );
            return rc;
        } else {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: load debug mode root ca success\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadDebugModeRootCert\0".as_ptr() as *const ::core::ffi::c_char,
                949i32,
            );
        }
        rc
    }
}