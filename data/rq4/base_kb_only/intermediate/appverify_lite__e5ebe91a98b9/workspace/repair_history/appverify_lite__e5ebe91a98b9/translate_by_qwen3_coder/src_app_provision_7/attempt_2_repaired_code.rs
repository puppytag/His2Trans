fn GetProfPermission(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfPermission) -> i32 {
    let jsonObj = unsafe { cJSON_GetObjectItem(root, b"permissions\0".as_ptr() as *const std::ffi::c_char) };
    if jsonObj.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: failed to get permissions\0".as_ptr() as *const std::ffi::c_char,
                b"GetProfPermission\0".as_ptr() as *const std::ffi::c_char,
                192i32,
            );
        }
        return crate::types::V_ERR as i32;
    }
    unsafe {
        (*profVal).permission = GetStringArrayTag(
            jsonObj as *const crate::types::cJSON,
            b"feature-permissions\0".as_ptr() as *const std::ffi::c_char,
            &mut (*profVal).permissionNum,
        );
        (*profVal).restricPermission = GetStringArrayTag(
            jsonObj as *const crate::types::cJSON,
            b"restricted-permissions\0".as_ptr() as *const std::ffi::c_char,
            &mut (*profVal).restricNum,
        );
    }
    crate::types::V_OK as i32
}