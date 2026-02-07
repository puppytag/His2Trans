pub extern "C" fn LoadCertAndCmpDest(certBase64: *const ::core::ffi::c_uchar, binSignCert: *const crate::types::CertInfo) -> i32 {
    if certBase64.is_null() || binSignCert.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let mut cert: crate::types::mbedtls_x509_crt = unsafe { std::mem::zeroed() };
    unsafe { crate::compat::mbedtls_x509_crt_init(&mut cert) };
    
    let cert_len = unsafe { libc::strlen(certBase64 as *const i8) } + 1;
    let ret = unsafe { crate::compat::mbedtls_x509_crt_parse(&mut cert, certBase64, cert_len as crate::types::size_t) };
    
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: load release cert failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
                846i32,
            )
        };
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
                847i32,
                certBase64,
            )
        };
        return crate::types::V_ERR as i32;
    }
    
    if crate::src_app_verify::CmpCert(&cert as *const _, binSignCert) == crate::types::V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: cert consistent\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
                852i32,
            )
        };
        unsafe { crate::compat::mbedtls_x509_crt_free(&mut cert) };
        return crate::types::V_OK as i32;
    }
    
    let _ = unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: cert inconsistent\0".as_ptr() as *const ::core::ffi::c_char,
            b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
            856i32,
        )
    };
    unsafe { crate::compat::mbedtls_x509_crt_free(&mut cert) };
    crate::types::V_ERR as i32
}