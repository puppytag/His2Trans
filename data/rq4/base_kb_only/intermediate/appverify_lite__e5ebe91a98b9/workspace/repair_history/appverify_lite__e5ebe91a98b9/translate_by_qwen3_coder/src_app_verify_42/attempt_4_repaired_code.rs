pub extern "C" fn APPVERI_SetDebugMode(mode: bool) -> i32 {
    unsafe {
        let mode_as_int = if mode { 1i32 } else { 0i32 };
        if crate::globals::g_isDebugMode == mode_as_int {
            return crate::types::V_OK as i32;
        }
        
        let ret = crate::src_mbedtls_pkcs7::PKCS7_EnableDebugMode(mode);
        if ret != crate::types::V_OK as i32 {
            return ret;
        }
        
        crate::globals::g_isDebugMode = mode_as_int;
        crate::types::V_OK as i32
    }
}