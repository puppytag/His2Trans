fn GetStringArrayTag(root: *const crate::types::cJSON, tag: *const std::ffi::c_char, numReturn: *mut i32) -> *mut *mut std::ffi::c_char {
    unsafe {
        let jsonObj = cJSON_GetObjectItem(root, tag);
        if jsonObj.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: failed to get %s\0".as_ptr() as *const _, __FUNCTION__!(), 85, tag);
            return std::ptr::null_mut();
        }
        let num = cJSON_GetArraySize(jsonObj);
        if num == 0 {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: array num 0\0".as_ptr() as *const _, __FUNCTION__!(), 90);
            *numReturn = 0;
            return std::ptr::null_mut();
        }
        let value = libc::malloc((num as usize) * std::mem::size_of::<*mut std::ffi::c_char>()) as *mut *mut std::ffi::c_char;
        if value.is_null() {
            let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: value is null\0".as_ptr() as *const _, __FUNCTION__!(), 96);
            *numReturn = 0;
            return std::ptr::null_mut();
        }
        let _ = memset_s(value as *mut _, (num as usize) * std::mem::size_of::<*mut std::ffi::c_char>(), 0, (num as usize) * std::mem::size_of::<*mut std::ffi::c_char>());

        for i in 0..num {
            let item = cJSON_GetArrayItem(jsonObj, i);
            if item.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: item is null\0".as_ptr() as *const _, __FUNCTION__!(), 104);
                crate::src_app_provision::FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
            if (*item).valuestring.is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: valuestring is NULL\0".as_ptr() as *const _, __FUNCTION__!(), 106);
                crate::src_app_provision::FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
            let len = libc::strlen((*item).valuestring);
            *value.offset(i as isize) = libc::malloc(len + 1) as *mut std::ffi::c_char;
            if (*value.offset(i as isize)).is_null() {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: value[i] is null\0".as_ptr() as *const _, __FUNCTION__!(), 112);
                crate::src_app_provision::FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
            let ret = strcpy_s(*value.offset(i as isize), len + 1, (*item).valuestring);
            if ret != 0 {
                let _ = HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: str cpy error : %d\0".as_ptr() as *const _, __FUNCTION__!(), 116, ret);
                crate::src_app_provision::FreeStringAttay(value, num);
                return std::ptr::null_mut();
            }
        }
        *numReturn = num;
        value
    }
}