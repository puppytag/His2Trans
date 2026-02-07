pub extern "C" fn HapPutData(hapBuffer: *const crate::types::HapBuf, offset: i32, data: *const ::core::ffi::c_uchar, len: i32) {
    if hapBuffer.is_null() {
        return;
    }
    
    let buffer = unsafe { (*hapBuffer).buffer };
    let buf_len = unsafe { (*hapBuffer).len };
    
    if buffer.is_null() {
        return;
    }
    
    if !data.is_null() && offset >= 0 && len > 0 && (buf_len - offset) >= len {
        let dest = unsafe { (buffer as *mut ::core::ffi::c_uchar).offset(offset as isize) };
        let dest_max = (buf_len - offset) as u32;
        let ret = unsafe { crate::compat::memcpy_s(dest as *mut ::core::ffi::c_void, dest_max, data as *const ::core::ffi::c_void, len as u32) };
        if ret != crate::types::EOK as i32 {
            unsafe {
                crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: memcpy_s fail\0".as_ptr() as *const ::core::ffi::c_char,
                    b"HapPutData\0".as_ptr() as *const ::core::ffi::c_char,
                    50i32,
                );
            }
        }
    }
}