pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    use crate::src_app_provision::*;
    use crate::compat::*;
    use crate::globals::*;
    use crate::types::*;
    const V_OK: i32 = 0;
    const V_ERR: i32 = 4294967295;
    if pf.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"pf\" is null\0".as_ptr() as *const i8, __FUNCTION__!(), 288);
        return V_ERR;
    }
    if buf.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"buf\" is null\0".as_ptr() as *const i8, __FUNCTION__!(), 289);
        return V_ERR;
    }
    unsafe {
        ProfInit(pf);
    }
    let pf_str = unsafe { libc::strchr(buf, '{' as i32) };
    if pf_str.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"pfStr\" is null\0".as_ptr() as *const i8, __FUNCTION__!(), 293);
        return V_ERR;
    }
    let root = unsafe { cJSON_Parse(pf_str) };
    if root.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"root\" is null\0".as_ptr() as *const i8, __FUNCTION__!(), 296);
        return V_ERR;
    }
    let json_obj = unsafe { cJSON_GetObjectItem(root, "version-code\0".as_ptr() as *const i8) };
    if json_obj.is_null() {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"jsonObj\" is null\0".as_ptr() as *const i8, __FUNCTION__!(), 299);
        unsafe { cJSON_Delete(root) };
        ProfFreeData(pf);
        return V_ERR;
    }
    unsafe {
        (*pf).versionCode = (*json_obj).valueint;
    }
    unsafe {
        (*pf).versionName = GetStringTag(root, "version-name\0".as_ptr() as *const i8);
    }
    if unsafe { (*pf).versionName.is_null() } {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"pf->versionName\" is null\0".as_ptr() as *const i8, __FUNCTION__!(), 303);
        unsafe { cJSON_Delete(root) };
        ProfFreeData(pf);
        return V_ERR;
    }
    unsafe {
        (*pf).uuid = GetStringTag(root, "uuid\0".as_ptr() as *const i8);
    }
    if unsafe { (*pf).uuid.is_null() } {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"pf->uuid\" is null\0".as_ptr() as *const i8, __FUNCTION__!(), 306);
        unsafe { cJSON_Delete(root) };
        ProfFreeData(pf);
        return V_ERR;
    }
    unsafe {
        (*pf).type_ = GetStringTag(root, "type\0".as_ptr() as *const i8);
    }
    if unsafe { (*pf).type_.is_null() } {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"pf->type\" is null\0".as_ptr() as *const i8, __FUNCTION__!(), 309);
        unsafe { cJSON_Delete(root) };
        ProfFreeData(pf);
        return V_ERR;
    }
    unsafe {
        (*pf).appDistType = GetStringTag(root, "app-distribution-type\0".as_ptr() as *const i8);
    }
    if unsafe { (*pf).appDistType.is_null() } {
        (*pf).appDistType = libc::malloc(1) as *mut i8;
        if unsafe { (*pf).appDistType.is_null() } {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"pf->appDistType\" is null\0".as_ptr() as *const i8, __FUNCTION__!(), 314);
            unsafe { cJSON_Delete(root) };
            ProfFreeData(pf);
            return V_ERR;
        }
        unsafe {
            *(*pf).appDistType = 0;
        }
    }
    let mut ret = unsafe { GetProfValidity(root, &mut (*pf).validity as *mut ProfValidity) };
    if ret != V_OK {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 319);
        unsafe { cJSON_Delete(root) };
        ProfFreeData(pf);
        return V_ERR;
    }
    ret = unsafe { GetProfBundleInfo(root, &mut (*pf).bundleInfo as *mut ProfBundleInfo) };
    if ret != V_OK {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 322);
        unsafe { cJSON_Delete(root) };
        ProfFreeData(pf);
        return V_ERR;
    }
    ret = unsafe { GetProfPermission(root, &mut (*pf).permission as *mut ProfPermission) };
    if ret != V_OK {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 325);
        unsafe { cJSON_Delete(root) };
        ProfFreeData(pf);
        return V_ERR;
    }
    ret = unsafe { GetProfDebugInfo(root, &mut (*pf).debugInfo as *mut ProfDebugInfo) };
    if ret != V_OK {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 328);
        unsafe { cJSON_Delete(root) };
        ProfFreeData(pf);
        return V_ERR;
    }
    ret = unsafe { GetProfIssuerInfo(root, pf) };
    if ret != V_OK {
        let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8, __FUNCTION__!(), 331);
        unsafe { cJSON_Delete(root) };
        ProfFreeData(pf);
        return V_ERR;
    }
    let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: \"parse profile json success\0".as_ptr() as *const i8, __FUNCTION__!(), 333);
    unsafe { cJSON_Delete(root) };
    V_OK
}