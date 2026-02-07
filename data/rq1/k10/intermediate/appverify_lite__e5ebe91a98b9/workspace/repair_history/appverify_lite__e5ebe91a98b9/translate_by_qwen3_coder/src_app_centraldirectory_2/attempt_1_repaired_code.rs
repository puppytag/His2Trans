pub extern "C" fn HapPutData(hapBuffer: *const crate::types::HapBuf, offset: i32, data: *const ::core::ffi::c_uchar, len: i32) {
    if hapBuffer.is_null() {
        return;
    }
    let hap = unsafe { &*hapBuffer };
    if hap.buffer.is_null() {
        return;
    }
    if !data.is_null() && offset >= 0 && len > 0 {
        let hap_len = hap.len;
        if (hap_len - offset) >= len {
            let dest = unsafe { (hap.buffer as *mut ::core::ffi::c_uchar).offset(offset as isize) };
            let dest_max = (hap_len - offset) as crate::types::size_t;
            let src = data;
            let count = len as crate::types::size_t;
            let ret = unsafe { crate::compat::memcpy_s(dest as *mut ::core::ffi::c_void, dest_max, src as *const ::core::ffi::c_void, count) };
            if ret != crate::types::EOK as i32 {
                let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const ::core::ffi::c_char, b"[%s:%d]: memcpy_s fail\0".as_ptr() as *const ::core::ffi::c_char, b"HapPutData\0".as_ptr() as *const ::core::ffi::c_char, 50) };
            }
        }
    }
}