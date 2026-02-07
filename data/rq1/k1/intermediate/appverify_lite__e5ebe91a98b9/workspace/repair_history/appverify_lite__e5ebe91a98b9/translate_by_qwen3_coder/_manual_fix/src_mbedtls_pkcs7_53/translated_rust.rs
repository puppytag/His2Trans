fn VerifyClicert(clicert: *mut crate::types::mbedtls_x509_crt, rootCert: *mut crate::types::mbedtls_x509_crt, pkcs7: *const crate::types::Pkcs7) -> i32 {
    let mut flags: u32 = 0;
    let crl_ptr = unsafe { &(*pkcs7).signedData.crl } as *const crate::types::mbedtls_x509_crl;
    let rc = unsafe {
        crate::compat::mbedtls_x509_crt_verify(
            clicert,
            rootCert,
            crl_ptr as *mut crate::types::mbedtls_x509_crl,
            std::ptr::null_mut(),
            &mut flags as *mut u32,
            None,
            None,
        )
    };
    if rc != 0 {
        crate::src_mbedtls_pkcs7::DLogCrtVerifyInfo(flags);
    } else {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: Verify signers cert chain root cert success\0".as_ptr() as *const i8,
        );
        if crate::src_mbedtls_pkcs7::VerifyCrl(clicert as *const crate::types::mbedtls_x509_crt, crl_ptr) != crate::types::PKCS7_SUCC as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: cert crl verify failed\0".as_ptr() as *const i8,
            );
            return crate::types::PKCS7_IS_REVOKED as i32;
        }
        return crate::types::PKCS7_SUCC as i32;
    }
    rc
}