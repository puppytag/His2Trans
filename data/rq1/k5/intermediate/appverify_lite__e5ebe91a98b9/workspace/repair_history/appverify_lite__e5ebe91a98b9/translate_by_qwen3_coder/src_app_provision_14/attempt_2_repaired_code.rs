pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    use crate::globals::*;
    use crate::compat::*;
    use crate::types::*;
    use ::core::ffi::c_void;
    use ::libc;
    if pf.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 288); }
        return V_ERR as i32;
    }
    if buf.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"buf\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 289); }
        return V_ERR as i32;
    }
    crate::src_app_provision::ProfInit(pf);
    let pf_str = unsafe { libc::strchr(buf, '{' as i32) };
    if pf_str.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pfStr\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 293); }
        return V_ERR as i32;
    }
    let root = unsafe { cJSON_Parse(pf_str) };
    if root.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"root\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 296); }
        return V_ERR as i32;
    }
    let json_obj = unsafe { cJSON_GetObjectItem(root, b"version-code\0".as_ptr() as *const _) };
    if json_obj.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"jsonObj\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 299); }
        unsafe { cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return V_ERR as i32;
    }
    unsafe { (*pf).versionCode = (*json_obj).valueint };
    unsafe { (*pf).versionName = crate::src_app_provision::GetStringTag(root, b"version-name\0".as_ptr() as *const _) };
    if unsafe { (*pf).versionName }.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->versionName\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 303); }
        unsafe { cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return V_ERR as i32;
    }
    unsafe { (*pf).uuid = crate::src_app_provision::GetStringTag(root, b"uuid\0".as_ptr() as *const _) };
    if unsafe { (*pf).uuid }.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->uuid\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 306); }
        unsafe { cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return V_ERR as i32;
    }
    unsafe { (*pf).type_ = crate::src_app_provision::GetStringTag(root, b"type\0".as_ptr() as *const _) };
    if unsafe { (*pf).type_ }.is_null() {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->type\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 309); }
        unsafe { cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return V_ERR as i32;
    }
    unsafe { (*pf).appDistType = crate::src_app_provision::GetStringTag(root, b"app-distribution-type\0".as_ptr() as *const _) };
    if unsafe { (*pf).appDistType }.is_null() {
        unsafe { (*pf).appDistType = libc::malloc(1) as *mut ::core::ffi::c_char };
        if unsafe { (*pf).appDistType }.is_null() {
            unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->appDistType\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 314); }
            unsafe { cJSON_Delete(root); }
            crate::src_app_provision::ProfFreeData(pf);
            return V_ERR as i32;
        }
        unsafe { *((*pf).appDistType) = 0 };
    }
    let mut ret = crate::src_app_provision::GetProfValidity(root, &mut unsafe { (*pf).validity } as *mut _);
    if ret != V_OK as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 319); }
        unsafe { cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return V_ERR as i32;
    }
    ret = crate::src_app_provision::GetProfBundleInfo(root, &mut unsafe { (*pf).bundleInfo } as *mut _);
    if ret != V_OK as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 322); }
        unsafe { cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return V_ERR as i32;
    }
    ret = crate::src_app_provision::GetProfPermission(root, &mut unsafe { (*pf).permission } as *mut _);
    if ret != V_OK as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 325); }
        unsafe { cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return V_ERR as i32;
    }
    ret = crate::src_app_provision::GetProfDebugInfo(root, &mut unsafe { (*pf).debugInfo } as *mut _);
    if ret != V_OK as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 328); }
        unsafe { cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return V_ERR as i32;
    }
    ret = crate::src_app_provision::GetProfIssuerInfo(root, pf);
    if ret != V_OK as i32 {
        unsafe { let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 331); }
        unsafe { cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return V_ERR as i32;
    }
    unsafe { let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"parse profile json success\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 333); }
    unsafe { cJSON_Delete(root); }
    V_OK as i32
}