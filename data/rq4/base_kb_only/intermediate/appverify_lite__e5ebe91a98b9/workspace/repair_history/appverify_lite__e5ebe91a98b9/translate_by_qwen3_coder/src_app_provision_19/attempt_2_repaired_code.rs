pub extern "C" fn VerifyProfileContent(pf: *const crate::types::ProfileProf) -> i32 {
    use crate::types::*;
    
    if pf.is_null() {
        return V_ERR as i32;
    }
    
    let ret = crate::src_app_provision::VerifyAppTypeAndDistribution(pf);
    if ret != V_OK as i32 {
        return V_ERR_INVALID_DISP_TYPE as i32;
    }
    
    let ret = crate::src_app_provision::VerifyAppBundleInfo(pf);
    if ret != V_OK as i32 {
        return V_ERR_INVALID_APP_BUNDLE as i32;
    }
    
    let ret = crate::src_app_provision::VerifyDebugInfo(pf);
    if ret != V_OK as i32 {
        return V_ERR_INVALID_DEVID as i32;
    }
    
    V_OK as i32
}