pub extern "C" fn APPVERI_SetDebugMode(mode: bool) -> i32 {
    unsafe {
        if crate::globals::g_isDebugMode == mode {
            return crate::types::V_OK as i32;
        }
        
        let ret = crate::src_mbedtls_pkcs7::PKCS7_EnableDebugMode(mode);
        if ret != crate::types::V_OK as i32 {
            return ret;
        }
        
        crate::globals::g_isDebugMode = mode;
        crate::types::V_OK as i32
    }
}