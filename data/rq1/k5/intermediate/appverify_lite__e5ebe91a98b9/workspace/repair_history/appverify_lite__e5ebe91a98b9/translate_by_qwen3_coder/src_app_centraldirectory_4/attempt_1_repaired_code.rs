pub extern "C" fn CreateHapBuffer(hapBuffer: *mut crate::types::HapBuf, len: i32) -> bool {
    if hapBuffer.is_null() || len <= 0 {
        unsafe {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: create buf fail, buf is null\0".as_ptr() as *const _,
                b"CreateHapBuffer\0".as_ptr() as *const _,
                73,
            );
        }
        return false;
    }
    unsafe {
        (*hapBuffer).buffer = libc::malloc(len as usize) as *mut ::core::ffi::c_void;
        if (*hapBuffer).buffer.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const _,
                b"[%s:%d]: create buf fail\0".as_ptr() as *const _,
                b"CreateHapBuffer\0".as_ptr() as *const _,
                78,
            );
            return false;
        }
        (*hapBuffer).len = len;
    }
    true
}