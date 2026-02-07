fn GetProfValidity(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfValidity) -> i32 {
    let jsonObj = unsafe { crate::compat::cJSON_GetObjectItem(root, "validity\0".as_ptr() as *const _) };
    if jsonObj.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const _, "[%s:%d]: failed to get validity\0".as_ptr() as *const _, __FUNCTION__, 132) };
        return crate::types::V_ERR as i32;
    }
    let notBefore = unsafe { crate::compat::cJSON_GetObjectItem(jsonObj, "not-before\0".as_ptr() as *const _) };
    if notBefore.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const _, "[%s:%d]: failed to get not-before\0".as_ptr() as *const _, __FUNCTION__, 138) };
        return crate::types::V_ERR as i32;
    }
    unsafe { (*profVal).notBefore = (*notBefore).valueint; }
    let notAfter = unsafe { crate::compat::cJSON_GetObjectItem(jsonObj, "not-after\0".as_ptr() as *const _) };
    if notAfter.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const _, "[%s:%d]: failed to get not-after\0".as_ptr() as *const _, __FUNCTION__, 145) };
        return crate::types::V_ERR as i32;
    }
    unsafe { (*profVal).notAfter = (*notAfter).valueint; }
    crate::types::V_OK as i32
}