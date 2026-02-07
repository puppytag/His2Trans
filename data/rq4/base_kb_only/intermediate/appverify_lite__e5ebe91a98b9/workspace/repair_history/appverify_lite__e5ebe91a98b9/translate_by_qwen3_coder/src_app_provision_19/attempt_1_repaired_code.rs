pub extern "C" fn VerifyProfileContent(pf: *const crate::types::ProfileProf) -> i32 {
    use crate::types::*;
    
    if pf.is_null() {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: pf is null\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                431i32,
            );
        }
        return V_ERR as i32;
    }
    
    let ret = crate::src_app_provision::VerifyAppTypeAndDistribution(pf);
    if ret != V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: invalid profile distribution type : %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                434i32,
                (*pf).appDistType,
            );
        }
        return V_ERR_INVALID_DISP_TYPE as i32;
    }
    
    let ret = crate::src_app_provision::VerifyAppBundleInfo(pf);
    if ret != V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: invalid profile app bundle info\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                439i32,
            );
        }
        return V_ERR_INVALID_APP_BUNDLE as i32;
    }
    
    let ret = crate::src_app_provision::VerifyDebugInfo(pf);
    if ret != V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: validate debug info\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyProfileContent\0".as_ptr() as *const ::core::ffi::c_char,
                445i32,
            );
        }
        return V_ERR_INVALID_DEVID as i32;
    }
    
    V_OK as i32
}