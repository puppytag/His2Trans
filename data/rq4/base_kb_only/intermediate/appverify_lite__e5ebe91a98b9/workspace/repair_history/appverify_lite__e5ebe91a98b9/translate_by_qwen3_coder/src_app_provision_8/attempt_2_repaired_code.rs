fn GetProfDebugInfo(root: *const crate::types::cJSON, profVal: *mut crate::types::ProfDebugInfo) -> i32 {
    let jsonObj = unsafe { cJSON_GetObjectItem(root, b"debug-info\0".as_ptr() as *const std::ffi::c_char) };
    if jsonObj.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: failed to get debug-info\0".as_ptr() as *const std::ffi::c_char,
                b"GetProfDebugInfo\0".as_ptr() as *const std::ffi::c_char,
                204 as i32,
            );
        }
        return crate::types::V_OK as i32;
    }
    
    let devIdType = crate::src_app_provision::GetStringTag(jsonObj as *const crate::types::cJSON, b"device-id-type\0".as_ptr() as *const std::ffi::c_char);
    unsafe { (*profVal).devIdType = devIdType; }
    
    if devIdType.is_null() {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: failed to get device-id-type\0".as_ptr() as *const std::ffi::c_char,
                b"GetProfDebugInfo\0".as_ptr() as *const std::ffi::c_char,
                209 as i32,
            );
        }
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