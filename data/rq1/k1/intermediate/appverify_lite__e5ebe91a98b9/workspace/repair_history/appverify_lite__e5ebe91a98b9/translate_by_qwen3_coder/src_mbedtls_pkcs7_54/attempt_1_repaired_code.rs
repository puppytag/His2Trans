pub extern "C" fn PKCS7_VerifyCertsChain(pkcs7: *const crate::types::Pkcs7) -> i32 {
    if pkcs7.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    let mut cnt: i32 = 0;
    let mut signer: *const crate::types::SignerInfo = unsafe { &(*pkcs7).signedData.signers };
    while !signer.is_null() {
        let clicert: *mut crate::types::mbedtls_x509_crt = unsafe { (*signer).certPath.crt };
        if clicert.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: Signer has no certs\0".as_ptr() as *const i8,
            );
            return crate::types::PKCS7_HAS_NO_SIGNER_CRT as i32;
        }
        let mut rc: i32;
        cnt += 1;
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: signer : %d\0".as_ptr() as *const i8,
        );
        unsafe {
            if crate::globals::g_debugModeEnabled != 0 {
                rc = crate::src_mbedtls_pkcs7::VerifyClicert(
                    clicert,
                    &mut crate::globals::g_debugModeRootCert as *mut i32 as *mut crate::types::mbedtls_x509_crt,
                    pkcs7,
                );
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_DEBUG,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: Verify inner: %d\0".as_ptr() as *const i8,
                );
                if rc == crate::types::PKCS7_SUCC as i32 {
                    signer = (*signer).next as *const crate::types::SignerInfo;
                    continue;
                }
                if rc == crate::types::PKCS7_IS_REVOKED as i32 {
                    return crate::types::PKCS7_IS_REVOKED as i32;
                }
            }
            rc = crate::src_mbedtls_pkcs7::VerifyClicert(
                clicert,
                (*signer).rootCert,
                pkcs7,
            );
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: Verify : %d\0".as_ptr() as *const i8,
            );
            if rc == crate::types::PKCS7_SUCC as i32 {
                signer = (*signer).next as *const crate::types::SignerInfo;
                continue;
            }
            if rc == crate::types::PKCS7_IS_REVOKED as i32 {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
            rc = crate::src_mbedtls_pkcs7::VerifyClicert(
                clicert,
                &mut crate::globals::g_ohosRootCert as *mut i32 as *mut crate::types::mbedtls_x509_crt,
                pkcs7,
            );
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_DEBUG,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: Verify self : %d\0".as_ptr() as *const i8,
            );
            if rc == crate::types::PKCS7_SUCC as i32 {
                signer = (*signer).next as *const crate::types::SignerInfo;
                continue;
            }
            return rc;
        }
    }
    crate::types::PKCS7_SUCC as i32
}