fn GetStringArrayTag(root: *const crate::types::cJSON, tag: *const std::ffi::c_char, numReturn: *mut i32) -> *mut *mut std::ffi::c_char {
    let jsonObj = unsafe { crate::compat::cJSON_GetObjectItem(root, tag) };
    if jsonObj.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: failed to get %s\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 85, tag) };
        return std::ptr::null_mut();
    }
    let num = unsafe { crate::compat::cJSON_GetArraySize(jsonObj) };
    if num == 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: array num 0\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 90) };
        unsafe { *numReturn = 0 };
        return std::ptr::null_mut();
    }
    let value = unsafe { libc::malloc((num as usize) * std::mem::size_of::<*mut std::ffi::c_char>()) as *mut *mut std::ffi::c_char };
    if value.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: value is null\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 96) };
        unsafe { *numReturn = 0 };
        return std::ptr::null_mut();
    }
    let byte_count = (num as usize) * std::mem::size_of::<*mut std::ffi::c_char>();
    let _ = unsafe { crate::compat::memset_s(value as *mut std::ffi::c_void, byte_count as u32, 0, byte_count as u32) };
    for i in 0..num {
        let item = unsafe { crate::compat::cJSON_GetArrayItem(jsonObj, i) };
        if item.is_null() {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: item is null\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 104) };
            crate::src_app_provision::FreeStringAttay(value, num);
            return std::ptr::null_mut();
        }
        let valuestring = unsafe { (*item).valuestring };
        if valuestring.is_null() {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: valuestring is NULL\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 106) };
            crate::src_app_provision::FreeStringAttay(value, num);
            return std::ptr::null_mut();
        }
        let len = unsafe { libc::strlen(valuestring) };
        unsafe { *value.offset(i as isize) = libc::malloc((len + 1) as usize) as *mut std::ffi::c_char };
        let val_i = unsafe { *value.offset(i as isize) };
        if val_i.is_null() {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: value[i] is null\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 112) };
            crate::src_app_provision::FreeStringAttay(value, num);
            return std::ptr::null_mut();
        }
        let ret = unsafe { crate::compat::strcpy_s(val_i, (len + 1) as u32, valuestring) };
        if ret != 0 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const std::ffi::c_char, b"[%s:%d]: str cpy error : %d\0".as_ptr() as *const std::ffi::c_char, b"GetStringArrayTag\0".as_ptr() as *const std::ffi::c_char, 116, ret) };
            crate::src_app_provision::FreeStringAttay(value, num);
            return std::ptr::null_mut();
        }
    }
    unsafe { *numReturn = num };
    value
}