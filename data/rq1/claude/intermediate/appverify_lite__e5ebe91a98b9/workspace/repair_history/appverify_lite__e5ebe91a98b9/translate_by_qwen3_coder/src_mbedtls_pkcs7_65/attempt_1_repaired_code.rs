pub extern "C" fn PKCS7_EnableDebugMode(mode: bool) -> i32 {
    unsafe {
        let mode_int: i32 = if mode { 1 } else { 0 };
        if crate::globals::g_debugModeEnabled == mode_int {
            return crate::types::PKCS7_SUCC as i32;
        }
        let rc: i32 = if mode {
            crate::src_mbedtls_pkcs7::LoadDebugModeRootCert()
        } else {
            crate::src_mbedtls_pkcs7::UnLoadDebugModeRootCert()
        };
        if rc != 0 {
            return rc;
        }
        crate::globals::g_debugModeEnabled = mode_int;
        crate::types::PKCS7_SUCC as i32
    }
}