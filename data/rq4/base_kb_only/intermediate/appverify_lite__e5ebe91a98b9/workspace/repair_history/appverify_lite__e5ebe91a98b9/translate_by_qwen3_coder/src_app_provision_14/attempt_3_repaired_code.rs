pub extern "C" fn ParseProfile(buf: *const ::core::ffi::c_char, len: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let _ = len;
    
    if pf.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    if buf.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    ProfInit(pf);
    
    let pf_str = unsafe { libc::strchr(buf, '{' as i32) };
    if pf_str.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let root = unsafe { cJSON_Parse(pf_str) };
    if root.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let version_code_str: &[u8] = b"version-code\0";
    let json_obj = unsafe { cJSON_GetObjectItem(root, version_code_str.as_ptr() as *const ::core::ffi::c_char) };
    if json_obj.is_null() {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    unsafe { (*pf).versionCode = (*json_obj).valueint; }
    
    let version_name_str: &[u8] = b"version-name\0";
    unsafe { (*pf).versionName = GetStringTag(root, version_name_str.as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).versionName.is_null() } {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    let uuid_str: &[u8] = b"uuid\0";
    unsafe { (*pf).uuid = GetStringTag(root, uuid_str.as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).uuid.is_null() } {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    let type_str: &[u8] = b"type\0";
    unsafe { (*pf).type_ = GetStringTag(root, type_str.as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).type_.is_null() } {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    let app_dist_type_str: &[u8] = b"app-distribution-type\0";
    unsafe { (*pf).appDistType = GetStringTag(root, app_dist_type_str.as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).appDistType.is_null() } {
        unsafe { (*pf).appDistType = libc::malloc(core::mem::size_of::<i8>()) as *mut i8; }
        if unsafe { (*pf).appDistType.is_null() } {
            unsafe { cJSON_Delete(root); ProfFreeData(pf); }
            return crate::types::V_ERR as i32;
        }
        unsafe { *(*pf).appDistType = 0; }
    }
    
    let mut ret = GetProfValidity(root, unsafe { &mut (*pf).validity as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    ret = GetProfBundleInfo(root, unsafe { &mut (*pf).bundleInfo as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    ret = GetProfPermission(root, unsafe { &mut (*pf).permission as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    ret = GetProfDebugInfo(root, unsafe { &mut (*pf).debugInfo as *mut _ });
    if ret != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    ret = GetProfIssuerInfo(root, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    unsafe { cJSON_Delete(root); }
    crate::types::V_OK as i32
}