fn GetStringTag(root: *const crate::types::cJSON, tag: *const std::ffi::c_char) -> *mut std::ffi::c_char {
    unsafe {
        let jsonObj = crate::compat::cJSON_GetObjectItem(root, tag);
        if jsonObj.is_null() || (*jsonObj).valuestring.is_null() {
            return std::ptr::null_mut();
        }
        let objLen = libc::strlen((*jsonObj).valuestring) as i32;
        if objLen < 0 {
            return std::ptr::null_mut();
        }
        let value = libc::malloc((objLen + 1) as usize) as *mut std::ffi::c_char;
        if value.is_null() {
            return std::ptr::null_mut();
        }
        let ret = crate::compat::strcpy_s(value, (objLen + 1) as u32, (*jsonObj).valuestring);
        if ret != 0 {
            if !value.is_null() {
                libc::free(value as *mut std::ffi::c_void);
            }
            return std::ptr::null_mut();
        }
        value
    }
}