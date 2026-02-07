fn UnLoadDebugModeRootCert() -> i32 {
    unsafe {
        crate::compat::mbedtls_x509_crt_free(
            std::ptr::addr_of_mut!(crate::globals::g_debugModeRootCert) as *mut crate::types::mbedtls_x509_crt
        );
    }
    crate::types::PKCS7_SUCC as i32
}