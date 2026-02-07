pub extern "C" fn VerifyProfileContent(pf: *const crate::types::ProfileProf) -> i32 {
    if pf.is_null() {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: pf is null\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                431,
            );
        }
        return crate::types::V_ERR as i32;
    }
    let mut ret = crate::src_app_provision::VerifyAppTypeAndDistribution(pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let app_dist_type = (*pf).appDistType;
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: invalid profile distribution type : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                434,
                app_dist_type,
            );
        }
        return crate::types::V_ERR_INVALID_DISP_TYPE as i32;
    }
    ret = crate::src_app_provision::VerifyAppBundleInfo(pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: invalid profile app bundle info\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                439,
            );
        }
        return crate::types::V_ERR_INVALID_APP_BUNDLE as i32;
    }
    ret = crate::src_app_provision::VerifyDebugInfo(pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: validate debug info\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                445,
            );
        }
        return crate::types::V_ERR_INVALID_DEVID as i32;
    }
    crate::types::V_OK as i32
}