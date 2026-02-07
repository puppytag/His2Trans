pub extern "C" fn PKCS7_EnableDebugMode(mode: bool) -> i32 {
    unsafe {
        if g_debugModeEnabled == mode {
            return PKCS7_SUCC as i32;
        }
        let rc = if mode {
            crate::src_mbedtls_pkcs7::LoadDebugModeRootCert()
        } else {
            crate::src_mbedtls_pkcs7::UnLoadDebugModeRootCert()
        };
        if rc != 0 {
            return rc;
        }
        g_debugModeEnabled = mode;
        PKCS7_SUCC as i32
    }
}