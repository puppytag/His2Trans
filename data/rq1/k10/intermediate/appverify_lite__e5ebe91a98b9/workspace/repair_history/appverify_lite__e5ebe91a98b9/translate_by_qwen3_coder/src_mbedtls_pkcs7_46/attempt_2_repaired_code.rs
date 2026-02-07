fn LoadDebugModeRootCert() -> i32 {
    unsafe {
        crate::compat::mbedtls_x509_crt_init(&mut crate::globals::g_debugModeRootCert as *mut _ as *mut crate::types::mbedtls_x509_crt);
        let rc = crate::compat::mbedtls_x509_crt_parse(
            &mut crate::globals::g_debugModeRootCert as *mut _ as *mut crate::types::mbedtls_x509_crt,
            std::ptr::null(),
            0 as crate::types::size_t,
        );
        if rc != 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: load debug mode root ca failed %d\0".as_ptr() as *const _,
                b"LoadDebugModeRootCert\0".as_ptr() as *const _,
                946,
                rc,
            );
            return rc;
        } else {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: load debug mode root ca success\0".as_ptr() as *const _,
                b"LoadDebugModeRootCert\0".as_ptr() as *const _,
                949,
            );
        }
        rc
    }
}