pub extern "C" fn LoadCertAndCmpDest(certBase64: *const ::core::ffi::c_uchar, binSignCert: *const crate::types::CertInfo) -> i32 {
    use crate::types::{V_ERR, V_OK, LOG_CORE, LOG_ERROR, LOG_INFO};
    if certBase64.is_null() || binSignCert.is_null() {
        return V_ERR as i32;
    }
    let mut cert: crate::types::mbedtls_x509_crt = unsafe { ::core::mem::zeroed() };
    unsafe {
        crate::compat::mbedtls_x509_crt_init(&mut cert);
    }
    let cert_cstr = certBase64 as *const ::core::ffi::c_char;
    let len = unsafe { libc::strlen(cert_cstr) } as crate::types::size_t;
    let ret = unsafe {
        crate::compat::mbedtls_x509_crt_parse(&mut cert, certBase64, len.wrapping_add(1))
    };
    if ret != V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE as u32,
                LOG_ERROR as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: load release cert failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
                846,
            )
        };
        let _ = unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE as u32,
                LOG_INFO as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
                847,
                certBase64,
            )
        };
        unsafe {
            crate::compat::mbedtls_x509_crt_free(&mut cert);
        }
        return V_ERR as i32;
    }
    if crate::src_app_verify::CmpCert(&cert, binSignCert) == V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE as u32,
                LOG_INFO as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: cert consistent\0".as_ptr() as *const ::core::ffi::c_char,
                b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
                852,
            )
        };
        unsafe {
            crate::compat::mbedtls_x509_crt_free(&mut cert);
        }
        return V_OK as i32;
    }
    let _ = unsafe {
        crate::compat::HiLogPrint(
            LOG_CORE as u32,
            LOG_ERROR as u32,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: cert inconsistent\0".as_ptr() as *const ::core::ffi::c_char,
            b"LoadCertAndCmpDest\0".as_ptr() as *const ::core::ffi::c_char,
            856,
        )
    };
    unsafe {
        crate::compat::mbedtls_x509_crt_free(&mut cert);
    }
    V_ERR as i32
}