pub extern "C" fn HapSetInt32(buffer: *const crate::types::HapBuf, offset: i32, value: i32) {
    if value < 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const ::core::ffi::c_char,
                "[%s:%d]: int32 value of out range: %d\0".as_ptr() as *const ::core::ffi::c_char,
                "HapSetInt32\0".as_ptr() as *const ::core::ffi::c_char,
                58,
                value,
            );
        }
        return;
    }
    if buffer.is_null() {
        return;
    }
    let buf = unsafe { &*buffer };
    if buf.buffer.is_null() {
        return;
    }
    if offset >= 0 && (buf.len - offset) >= (::core::mem::size_of::<i32>() as i32) {
        let ptr = unsafe { buf.buffer.offset(offset as isize) as *mut ::core::ffi::c_uchar };
        crate::src_app_common::HapPutInt32(ptr, buf.len - offset, value);
    }
}