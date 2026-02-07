pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    use crate::src_app_provision::*;
    use crate::types::*;
    use crate::globals::*;
    use crate::compat::*;
    use ::core::ffi::c_char;
    use libc::{strchr, malloc};

    if pf.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"pf\" is null\0".as_ptr() as *const c_char, __FUNCTION__, 288);
        return V_ERR as i32;
    }
    if buf.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"buf\" is null\0".as_ptr() as *const c_char, __FUNCTION__, 289);
        return V_ERR as i32;
    }
    unsafe {
        ProfInit(pf);
    }
    let pf_str = unsafe { strchr(buf, '{' as i32) };
    if pf_str.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"pfStr\" is null\0".as_ptr() as *const c_char, __FUNCTION__, 293);
        return V_ERR as i32;
    }
    let root = unsafe { cJSON_Parse(pf_str) };
    if root.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"root\" is null\0".as_ptr() as *const c_char, __FUNCTION__, 296);
        return V_ERR as i32;
    }
    let json_obj = unsafe { cJSON_GetObjectItem(root, "version-code\0".as_ptr() as *const c_char) };
    if json_obj.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"jsonObj\" is null\0".as_ptr() as *const c_char, __FUNCTION__, 299);
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    unsafe {
        (*pf).versionCode = (*json_obj).valueint;
    }
    unsafe {
        (*pf).versionName = GetStringTag(root, "version-name\0".as_ptr() as *const c_char);
    }
    if unsafe { (*pf).versionName.is_null() } {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"pf->versionName\" is null\0".as_ptr() as *const c_char, __FUNCTION__, 303);
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    unsafe {
        (*pf).uuid = GetStringTag(root, "uuid\0".as_ptr() as *const c_char);
    }
    if unsafe { (*pf).uuid.is_null() } {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"pf->uuid\" is null\0".as_ptr() as *const c_char, __FUNCTION__, 306);
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    unsafe {
        (*pf).type_ = GetStringTag(root, "type\0".as_ptr() as *const c_char);
    }
    if unsafe { (*pf).type_.is_null() } {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"pf->type\" is null\0".as_ptr() as *const c_char, __FUNCTION__, 309);
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    unsafe {
        (*pf).appDistType = GetStringTag(root, "app-distribution-type\0".as_ptr() as *const c_char);
    }
    if unsafe { (*pf).appDistType.is_null() } {
        (*pf).appDistType = malloc(1) as *mut c_char;
        if unsafe { (*pf).appDistType.is_null() } {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"pf->appDistType\" is null\0".as_ptr() as *const c_char, __FUNCTION__, 314);
            unsafe { cJSON_Delete(root); }
            unsafe { ProfFreeData(pf); }
            return V_ERR as i32;
        }
        unsafe {
            *((*pf).appDistType) = 0;
        }
    }
    let mut ret = unsafe { GetProfValidity(root, &mut (*pf).validity as *mut ProfValidity) };
    if ret != V_OK as i32 {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const c_char, __FUNCTION__, 319);
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    ret = unsafe { GetProfBundleInfo(root, &mut (*pf).bundleInfo as *mut ProfBundleInfo) };
    if ret != V_OK as i32 {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const c_char, __FUNCTION__, 322);
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    ret = unsafe { GetProfPermission(root, &mut (*pf).permission as *mut ProfPermission) };
    if ret != V_OK as i32 {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const c_char, __FUNCTION__, 325);
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    ret = unsafe { GetProfDebugInfo(root, &mut (*pf).debugInfo as *mut ProfDebugInfo) };
    if ret != V_OK as i32 {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const c_char, __FUNCTION__, 328);
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    ret = unsafe { GetProfIssuerInfo(root, pf) };
    if ret != V_OK as i32 {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const c_char, __FUNCTION__, 331);
        unsafe { cJSON_Delete(root); }
        unsafe { ProfFreeData(pf); }
        return V_ERR as i32;
    }
    let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const c_char, "[%s:%d]: \"parse profile json success\0".as_ptr() as *const c_char, __FUNCTION__, 333);
    unsafe { cJSON_Delete(root); }
    V_OK as i32
}