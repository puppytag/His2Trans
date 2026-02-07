pub extern "C" fn APPVERI_SetDebugMode(mode: bool) -> i32 {
    unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: set debug mode: %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"APPVERI_SetDebugMode\0".as_ptr() as *const ::core::ffi::c_char,
            1227i32,
            mode as i32,
        );
    }
    
    let g_debug_mode_val = unsafe { crate::globals::g_isDebugMode };
    if (g_debug_mode_val != 0) == mode {
        return crate::types::V_OK as i32;
    }
    
    let ret = crate::src_mbedtls_pkcs7::PKCS7_EnableDebugMode(mode);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: enable pcks7 debug mode failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"APPVERI_SetDebugMode\0".as_ptr() as *const ::core::ffi::c_char,
                1233i32,
            );
        }
        return ret;
    }
    
    unsafe {
        crate::globals::g_isDebugMode = mode as i32;
    }
    
    crate::types::V_OK as i32
}