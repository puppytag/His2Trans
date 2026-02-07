pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    use crate::globals::*;
    use crate::compat::*;
    unsafe {
        if pf.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 288);
            return V_ERR as i32;
        }
        if buf.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"buf\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 289);
            return V_ERR as i32;
        }
        crate::src_app_provision::ProfInit(pf);
        let pf_str = libc::strchr(buf, '{' as i32);
        if pf_str.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pfStr\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 293);
            return V_ERR as i32;
        }
        let root = cJSON_Parse(pf_str);
        if root.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"root\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 296);
            return V_ERR as i32;
        }
        let json_obj = cJSON_GetObjectItem(root, b"version-code\0".as_ptr() as *const _);
        if json_obj.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"jsonObj\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 299);
            cJSON_Delete(root);
            crate::src_app_provision::ProfFreeData(pf);
            return V_ERR as i32;
        }
        (*pf).versionCode = (*json_obj).valueint;
        (*pf).versionName = crate::src_app_provision::GetStringTag(root, b"version-name\0".as_ptr() as *const _);
        if (*pf).versionName.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->versionName\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 303);
            cJSON_Delete(root);
            crate::src_app_provision::ProfFreeData(pf);
            return V_ERR as i32;
        }
        (*pf).uuid = crate::src_app_provision::GetStringTag(root, b"uuid\0".as_ptr() as *const _);
        if (*pf).uuid.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->uuid\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 306);
            cJSON_Delete(root);
            crate::src_app_provision::ProfFreeData(pf);
            return V_ERR as i32;
        }
        (*pf).type_ = crate::src_app_provision::GetStringTag(root, b"type\0".as_ptr() as *const _);
        if (*pf).type_.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->type\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 309);
            cJSON_Delete(root);
            crate::src_app_provision::ProfFreeData(pf);
            return V_ERR as i32;
        }
        (*pf).appDistType = crate::src_app_provision::GetStringTag(root, b"app-distribution-type\0".as_ptr() as *const _);
        if (*pf).appDistType.is_null() {
            (*pf).appDistType = libc::malloc(1) as *mut ::core::ffi::c_char;
            if (*pf).appDistType.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"pf->appDistType\" is null\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 314);
                cJSON_Delete(root);
                crate::src_app_provision::ProfFreeData(pf);
                return V_ERR as i32;
            }
            *(*pf).appDistType = 0;
        }
        let mut ret = crate::src_app_provision::GetProfValidity(root, &mut (*pf).validity as *mut _);
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 319);
            cJSON_Delete(root);
            crate::src_app_provision::ProfFreeData(pf);
            return V_ERR as i32;
        }
        ret = crate::src_app_provision::GetProfBundleInfo(root, &mut (*pf).bundleInfo as *mut _);
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 322);
            cJSON_Delete(root);
            crate::src_app_provision::ProfFreeData(pf);
            return V_ERR as i32;
        }
        ret = crate::src_app_provision::GetProfPermission(root, &mut (*pf).permission as *mut _);
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 325);
            cJSON_Delete(root);
            crate::src_app_provision::ProfFreeData(pf);
            return V_ERR as i32;
        }
        ret = crate::src_app_provision::GetProfDebugInfo(root, &mut (*pf).debugInfo as *mut _);
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 328);
            cJSON_Delete(root);
            crate::src_app_provision::ProfFreeData(pf);
            return V_ERR as i32;
        }
        ret = crate::src_app_provision::GetProfIssuerInfo(root, pf);
        if ret != V_OK as i32 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 331);
            cJSON_Delete(root);
            crate::src_app_provision::ProfFreeData(pf);
            return V_ERR as i32;
        }
        let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: \"parse profile json success\0".as_ptr() as *const _, b"ParseProfile\0".as_ptr() as *const _, 333);
        cJSON_Delete(root);
        return V_OK as i32;
    }
}