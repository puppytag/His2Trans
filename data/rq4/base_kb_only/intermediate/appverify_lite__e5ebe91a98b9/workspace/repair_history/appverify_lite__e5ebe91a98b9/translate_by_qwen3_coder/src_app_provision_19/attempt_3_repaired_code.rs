pub extern "C" fn VerifyProfileContent(pf: *const crate::types::ProfileProf) -> i32 {
    if pf.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let ret = crate::src_app_provision::VerifyAppTypeAndDistribution(pf);
    if ret != crate::types::V_OK as i32 {
        return crate::types::V_ERR_INVALID_DISP_TYPE as i32;
    }
    
    let ret = crate::src_app_provision::VerifyAppBundleInfo(pf);
    if ret != crate::types::V_OK as i32 {
        return crate::types::V_ERR_INVALID_APP_BUNDLE as i32;
    }
    
    let ret = crate::src_app_provision::VerifyDebugInfo(pf);
    if ret != crate::types::V_OK as i32 {
        return crate::types::V_ERR_INVALID_DEVID as i32;
    }
    
    let _ = ret;
    crate::types::V_OK as i32
}