fn ParseCertGetPk(certEncoded: *const std::ffi::c_char, pk: *mut crate::types::AppSignPk) -> i32 {
    use crate::types::{mbedtls_x509_crt, V_OK, V_ERR};
    unsafe {
        let cert = libc::malloc(std::mem::size_of::<mbedtls_x509_crt>()) as *mut mbedtls_x509_crt;
        if cert.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: cert is null\0".as_ptr() as *const _,
                b"ParseCertGetPk\0".as_ptr() as *const _,
                657,
            );
            return V_ERR as i32;
        }
        crate::compat::mbedtls_x509_crt_init(cert);
        let cert_len = (libc::strlen(certEncoded) + 1) as usize;
        let ret = crate::compat::mbedtls_x509_crt_parse(
            cert,
            certEncoded as *const u8,
            cert_len as crate::types::size_t,
        );
        if ret != V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: load cert failed, ret: %d\0".as_ptr() as *const _,
                b"ParseCertGetPk\0".as_ptr() as *const _,
                662,
                ret,
            );
            crate::compat::mbedtls_x509_crt_free(cert);
            libc::free(cert as *mut _);
            return V_ERR as i32;
        }
        let mut len: i32 = 0;
        let pk_buf = crate::src_app_verify::GetPkBuf(&(*cert).pk, &mut len);
        if pk_buf.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: get pk error\0".as_ptr() as *const _,
                b"ParseCertGetPk\0".as_ptr() as *const _,
                669,
            );
            crate::compat::mbedtls_x509_crt_free(cert);
            libc::free(cert as *mut _);
            return V_ERR as i32;
        }
        (*pk).pk = pk_buf as *mut std::ffi::c_char;
        (*pk).len = len;
        crate::compat::mbedtls_x509_crt_free(cert);
        libc::free(cert as *mut _);
        V_OK as i32
    }
}