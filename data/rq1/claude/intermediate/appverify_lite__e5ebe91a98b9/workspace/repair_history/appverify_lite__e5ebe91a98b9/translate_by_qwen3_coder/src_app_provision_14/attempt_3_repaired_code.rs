pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let _ = len;
    if pf.is_null() {
        return crate::types::V_ERR as i32;
    }
    if buf.is_null() {
        return crate::types::V_ERR as i32;
    }
    crate::src_app_provision::ProfInit(pf);
    let pfStr = unsafe { libc::strchr(buf, '{' as i32) };
    if pfStr.is_null() {
        return crate::types::V_ERR as i32;
    }
    let root = unsafe { crate::compat::cJSON_Parse(pfStr) };
    if root.is_null() {
        return crate::types::V_ERR as i32;
    }
    let jsonObj = unsafe { crate::compat::cJSON_GetObjectItem(root, b"version-code\0".as_ptr() as *const ::core::ffi::c_char) };
    if jsonObj.is_null() {
        unsafe { crate::compat::cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR as i32;
    }
    unsafe { (*pf).versionCode = (*jsonObj).valueint; }
    unsafe { (*pf).versionName = crate::src_app_provision::GetStringTag(root, b"version-name\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).versionName.is_null() } {
        unsafe { crate::compat::cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR as i32;
    }
    unsafe { (*pf).uuid = crate::src_app_provision::GetStringTag(root, b"uuid\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).uuid.is_null() } {
        unsafe { crate::compat::cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR as i32;
    }
    unsafe { (*pf).type_ = crate::src_app_provision::GetStringTag(root, b"type\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).type_.is_null() } {
        unsafe { crate::compat::cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR as i32;
    }
    unsafe { (*pf).appDistType = crate::src_app_provision::GetStringTag(root, b"app-distribution-type\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).appDistType.is_null() } {
        unsafe { (*pf).appDistType = libc::malloc(core::mem::size_of::<::core::ffi::c_char>()) as *mut ::core::ffi::c_char; }
        if unsafe { (*pf).appDistType.is_null() } {
            unsafe { crate::compat::cJSON_Delete(root); }
            crate::src_app_provision::ProfFreeData(pf);
            return crate::types::V_ERR as i32;
        }
        unsafe { *(*pf).appDistType = 0; }
    }
    let mut ret = crate::src_app_provision::GetProfValidity(root, unsafe { &mut (*pf).validity as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR as i32;
    }
    ret = crate::src_app_provision::GetProfBundleInfo(root, unsafe { &mut (*pf).bundleInfo as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR as i32;
    }
    ret = crate::src_app_provision::GetProfPermission(root, unsafe { &mut (*pf).permission as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR as i32;
    }
    ret = crate::src_app_provision::GetProfDebugInfo(root, unsafe { &mut (*pf).debugInfo as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR as i32;
    }
    ret = crate::src_app_provision::GetProfIssuerInfo(root, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe { crate::compat::cJSON_Delete(root); }
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR as i32;
    }
    unsafe { crate::compat::cJSON_Delete(root); }
    crate::types::V_OK as i32
}