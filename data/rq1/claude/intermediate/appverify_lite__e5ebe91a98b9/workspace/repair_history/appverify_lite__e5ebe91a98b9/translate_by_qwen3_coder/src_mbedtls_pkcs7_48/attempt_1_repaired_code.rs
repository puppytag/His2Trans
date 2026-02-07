fn LoadSelfSignedCert() -> i32 {
    unsafe {
        crate::compat::mbedtls_x509_crt_init(&mut crate::globals::g_ohosRootCert);
        let rc = crate::compat::mbedtls_x509_crt_parse(
            &mut crate::globals::g_ohosRootCert,
            crate::globals::OHOS_ROOT_CERT_IN_PEM!().as_ptr(),
            std::mem::size_of_val(&crate::globals::OHOS_ROOT_CERT_IN_PEM!()) as usize,
        );
        if rc != 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: load self signed ca failed %d\0".as_ptr() as *const i8,
                b"LoadSelfSignedCert\0".as_ptr() as *const i8,
                964i32,
                rc,
            );
            return rc;
        } else {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: load self signed root ca success\0".as_ptr() as *const i8,
                b"LoadSelfSignedCert\0".as_ptr() as *const i8,
                967i32,
            );
        }
        rc
    }
}