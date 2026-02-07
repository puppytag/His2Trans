Looking at the error, it's occurring at `src/src_app_verify.rs:373:55` - the error says `no method named 'as_ptr' found for type 'i32'`. This suggests there's code elsewhere in the file that has this issue, not in the `ParseProfile` function I'm translating.

However, looking at the previous attempts, the issue seems to be with the syntax `(version_code_str.as_ptr)()` which is incorrect. The correct syntax is `version_code_str.as_ptr()`.

Let me provide a clean translation:

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
    
    let json_obj = unsafe { cJSON_GetObjectItem(root, b"version-code\0".as_ptr() as *const ::core::ffi::c_char) };
    if json_obj.is_null() {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    unsafe { (*pf).versionCode = (*json_obj).valueint; }
    
    unsafe { (*pf).versionName = GetStringTag(root, b"version-name\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).versionName.is_null() } {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    unsafe { (*pf).uuid = GetStringTag(root, b"uuid\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).uuid.is_null() } {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    unsafe { (*pf).type_ = GetStringTag(root, b"type\0".as_ptr() as *const ::core::ffi::c_char); }
    if unsafe { (*pf).type_.is_null() } {
        unsafe { cJSON_Delete(root); ProfFreeData(pf); }
        return crate::types::V_ERR as i32;
    }
    
    unsafe { (*pf).appDistType = GetStringTag(root, b"app-distribution-type\0".as_ptr() as *const ::core::ffi::c_char); }
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