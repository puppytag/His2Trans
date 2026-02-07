pub extern "C" fn LoadCertAndCmpDest(certBase64: *const ::core::ffi::c_uchar, binSignCert: *const crate::types::CertInfo) -> i32 {
    use crate::types::*;
    use crate::compat::*;
    
    if certBase64.is_null() || binSignCert.is_null() {
        return V_ERR as i32;
    }
    
    let mut cert: mbedtls_x509_crt = unsafe { std::mem::zeroed() };
    unsafe { mbedtls_x509_crt_init(&mut cert) };
    
    let cert_len = unsafe { libc::strlen(certBase64 as *const i8) + 1 };
    let ret = unsafe { mbedtls_x509_crt_parse(&mut cert, certBase64, cert_len as size_t) };
    
    if ret != V_OK as i32 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: load release cert failed\0".as_ptr() as *const i8,
                b"LoadCertAndCmpDest\0".as_ptr() as *const i8,
                846 as i32,
            );
            HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: %s\0".as_ptr() as *const i8,
                b"LoadCertAndCmpDest\0".as_ptr() as *const i8,
                847 as i32,
                certBase64,
            );
        }
        return V_ERR as i32;
    }
    
    if crate::src_app_verify::CmpCert(&cert as *const mbedtls_x509_crt, binSignCert) == V_OK as i32 {
        unsafe {
            HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: cert consistent\0".as_ptr() as *const i8,
                b"LoadCertAndCmpDest\0".as_ptr() as *const i8,
                852 as i32,
            );
            mbedtls_x509_crt_free(&mut cert);
        }
        return V_OK as i32;
    }
    
    unsafe {
        HiLogPrint(
            LOG_CORE,
            LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: cert inconsistent\0".as_ptr() as *const i8,
            b"LoadCertAndCmpDest\0".as_ptr() as *const i8,
            856 as i32,
        );
        mbedtls_x509_crt_free(&mut cert);
    }
    
    V_ERR as i32
}