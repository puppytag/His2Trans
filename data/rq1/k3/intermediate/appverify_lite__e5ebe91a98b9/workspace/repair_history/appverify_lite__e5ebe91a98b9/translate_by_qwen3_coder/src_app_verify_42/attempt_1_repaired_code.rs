pub extern "C" fn APPVERI_SetDebugMode(mode: bool) -> i32 {
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const _,
            b"[%s:%d]: set debug mode: %d\0".as_ptr() as *const _,
            b"APPVERI_SetDebugMode\0".as_ptr() as *const _,
            1227,
            mode as i32,
        );
        if g_isDebugMode == mode {
            return crate::types::V_OK as i32;
        }
    }
    let ret = crate::src_mbedtls_pkcs7::PKCS7_EnableDebugMode(mode);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: enable pcks7 debug mode failed\0".as_ptr() as *const _,
                b"APPVERI_SetDebugMode\0".as_ptr() as *const _,
                1233,
            );
        }
        return ret;
    }
    unsafe {
        g_isDebugMode = mode;
    }
    crate::types::V_OK as i32
}