fn ParseCertGetPk(certEncoded: *const core::ffi::c_char, pk: *mut crate::types::AppSignPk) -> i32 {
    unsafe {
        let cert = libc::malloc(std::mem::size_of::<crate::types::mbedtls_x509_crt>()) as *mut crate::types::mbedtls_x509_crt;
        if cert.is_null() {
            return crate::types::V_ERR as i32;
        }

        crate::compat::mbedtls_x509_crt_init(cert);
        
        let cert_len = libc::strlen(certEncoded) + 1;
        let ret = crate::compat::mbedtls_x509_crt_parse(
            cert,
            certEncoded as *const u8,
            cert_len as crate::types::size_t,
        );
        
        if ret != crate::types::V_OK as i32 {
            libc::free(cert as *mut core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        let mut len: i32 = 0;
        let pk_ptr = &(*cert).pk as *const crate::types::mbedtls_pk_context;
        let pkBuf = crate::src_app_verify::GetPkBuf(pk_ptr, &mut len);
        
        if pkBuf.is_null() {
            crate::compat::mbedtls_x509_crt_free(cert);
            libc::free(cert as *mut core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }
        
        (*pk).pk = pkBuf as *mut core::ffi::c_char;
        (*pk).len = len;
        
        crate::compat::mbedtls_x509_crt_free(cert);
        libc::free(cert as *mut core::ffi::c_void);
        
        crate::types::V_OK as i32
    }
}