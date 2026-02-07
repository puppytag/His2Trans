fn GetProfDebugInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfDebugInfo) -> i32 {
    let jsonObj = unsafe { cJSON_GetObjectItem(root, b"debug-info\0".as_ptr() as *const std::ffi::c_char) };
    if jsonObj.is_null() {
        return crate::types::V_OK as i32;
    }
    
    let devIdType = crate::src_app_provision::GetStringTag(jsonObj as *const crate::types::cJSON, b"device-id-type\0".as_ptr() as *const std::ffi::c_char);
    unsafe { (*profVal).devIdType = devIdType; }
    
    if devIdType.is_null() {
        return crate::types::V_OK as i32;
    }
    
    unsafe {
        (*profVal).deviceId = crate::src_app_provision::GetStringArrayTag(
            jsonObj as *const crate::types::cJSON,
            b"device-ids\0".as_ptr() as *const std::ffi::c_char,
            &mut (*profVal).devidNum,
        );
    }
    
    crate::types::V_OK as i32
}