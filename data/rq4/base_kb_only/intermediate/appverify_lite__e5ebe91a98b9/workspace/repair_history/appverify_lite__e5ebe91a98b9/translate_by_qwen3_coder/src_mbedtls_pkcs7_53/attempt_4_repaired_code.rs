fn VerifyClicert(clicert: *mut crate::types::mbedtls_x509_crt, rootCert: *mut crate::types::mbedtls_x509_crt, pkcs7: *const crate::types::Pkcs7) -> i32 {
    let mut flags: u32 = 0;
    
    let crl_ptr = unsafe { &(*pkcs7).signedData.crl as *const crate::types::mbedtls_x509_crl as *mut crate::types::mbedtls_x509_crl };
    
    let rc = unsafe {
        crate::compat::mbedtls_x509_crt_verify(
            clicert,
            rootCert,
            crl_ptr,
            std::ptr::null_mut(),
            &mut flags,
            None,
            std::ptr::null_mut()
        )
    };
    
    if rc != 0 {
        crate::src_mbedtls_pkcs7::DLogCrtVerifyInfo(flags);
    } else {
        let appverify_bytes: &[u8] = b"appverify\0";
        let success_bytes: &[u8] = b"[%s:%d]: Verify signers cert chain root cert success\0";
        let func_bytes: &[u8] = b"VerifyClicert\0";
        
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                (appverify_bytes.as_ptr)() as *const ::core::ffi::c_char,
                (success_bytes.as_ptr)() as *const ::core::ffi::c_char,
                (func_bytes.as_ptr)() as *const ::core::ffi::c_char,
                1029i32
            );
        }
        
        let crl_const_ptr = unsafe { &(*pkcs7).signedData.crl as *const crate::types::mbedtls_x509_crl };
        
        if crate::src_mbedtls_pkcs7::VerifyCrl(clicert as *const crate::types::mbedtls_x509_crt, crl_const_ptr) != crate::types::PKCS7_SUCC as i32 {
            let fail_bytes: &[u8] = b"[%s:%d]: cert crl verify failed\0";
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    (appverify_bytes.as_ptr)() as *const ::core::ffi::c_char,
                    (fail_bytes.as_ptr)() as *const ::core::ffi::c_char,
                    (func_bytes.as_ptr)() as *const ::core::ffi::c_char,
                    1031i32
                );
            }
            return crate::types::PKCS7_IS_REVOKED as i32;
        }
        return crate::types::PKCS7_SUCC as i32;
    }
    
    rc
}