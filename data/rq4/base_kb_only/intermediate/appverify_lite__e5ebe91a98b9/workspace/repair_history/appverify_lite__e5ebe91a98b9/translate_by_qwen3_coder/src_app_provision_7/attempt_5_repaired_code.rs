fn GetProfPermission(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfPermission) -> i32 {
    let jsonObj = unsafe { cJSON_GetObjectItem(root, b"permissions\0".as_ptr() as *const std::ffi::c_char) };
    if jsonObj.is_null() {
        return crate::types::V_ERR as i32;
    }
    unsafe {
        (*profVal).permission = crate::src_app_provision::GetStringArrayTag(
            jsonObj as *const crate::types::cJSON,
            b"feature-permissions\0".as_ptr() as *const std::ffi::c_char,
            &mut (*profVal).permissionNum,
        );
        (*profVal).restricPermission = crate::src_app_provision::GetStringArrayTag(
            jsonObj as *const crate::types::cJSON,
            b"restricted-permissions\0".as_ptr() as *const std::ffi::c_char,
            &mut (*profVal).restricNum,
        );
    }
    crate::types::V_OK as i32
}