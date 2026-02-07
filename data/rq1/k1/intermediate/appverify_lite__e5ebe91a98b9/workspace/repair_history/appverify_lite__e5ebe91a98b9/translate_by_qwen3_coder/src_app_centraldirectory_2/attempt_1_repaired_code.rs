pub extern "C" fn HapPutData(hapBuffer: *const crate::types::HapBuf, offset: i32, data: *const ::core::ffi::c_uchar, len: i32) {
    if hapBuffer.is_null() {
        return;
    }
    let hap = unsafe { &*hapBuffer };
    if hap.buffer.is_null() {
        return;
    }
    if !data.is_null() && offset >= 0 && len > 0 {
        let remaining = hap.len - offset;
        if remaining >= len {
            let dest = unsafe { (hap.buffer as *mut ::core::ffi::c_uchar).offset(offset as isize) };
            let ret = unsafe { crate::compat::memcpy_s(dest as *mut ::core::ffi::c_void, remaining as crate::types::size_t, data as *const ::core::ffi::c_void, len as crate::types::size_t) };
            if ret != crate::types::EOK as i32 {
                let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: memcpy_s fail\0".as_ptr() as *const ::core::ffi::c_char, b"HapPutData\0".as_ptr() as *const ::core::ffi::c_char, 50) };
            }
        }
    }
}