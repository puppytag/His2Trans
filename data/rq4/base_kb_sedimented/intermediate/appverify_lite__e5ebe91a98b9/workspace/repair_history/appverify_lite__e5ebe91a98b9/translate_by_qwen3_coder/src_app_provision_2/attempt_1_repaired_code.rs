fn GetStringTag(root: *const crate::types::cJSON, tag: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    let jsonObj = unsafe { cJSON_GetObjectItem(root, tag) };
    if jsonObj.is_null() || unsafe { (*jsonObj).valuestring.is_null() } {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: failed to get %s\0".as_ptr() as *const std::ffi::c_char,
                b"GetStringTag\0".as_ptr() as *const std::ffi::c_char,
                45i32,
                tag,
            );
        }
        return std::ptr::null_mut();
    }
    let objLen: i32 = unsafe { libc::strlen((*jsonObj).valuestring) } as i32;
    if objLen < 0 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: len error\0".as_ptr() as *const std::ffi::c_char,
                b"GetStringTag\0".as_ptr() as *const std::ffi::c_char,
                50i32,
            );
        }
        return std::ptr::null_mut();
    }
    let value: *mut std::ffi::c_char = unsafe { libc::malloc((objLen + 1) as usize) } as *mut std::ffi::c_char;
    if value.is_null() {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: malloc error: %d\0".as_ptr() as *const std::ffi::c_char,
                b"GetStringTag\0".as_ptr() as *const std::ffi::c_char,
                55i32,
                objLen + 1,
            );
        }
        return std::ptr::null_mut();
    }
    let ret: crate::types::errno_t = unsafe { strcpy_s(value, (objLen + 1) as u32, (*jsonObj).valuestring) };
    if ret != 0 {
        if !value.is_null() {
            unsafe { libc::free(value as *mut std::ffi::c_void) };
        }
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: strcpy error: %d\0".as_ptr() as *const std::ffi::c_char,
                b"GetStringTag\0".as_ptr() as *const std::ffi::c_char,
                61i32,
                ret,
            );
        }
        return std::ptr::null_mut();
    }
    value
}