pub extern "C" fn HapPutData(hapBuffer: *const crate::types::HapBuf, offset: i32, data: *const ::core::ffi::c_uchar, len: i32) {
    if hapBuffer.is_null() {
        return;
    }
    let hap = unsafe { &*hapBuffer };
    if hap.buffer.is_null() {
        return;
    }
    if !data.is_null() && offset >= 0 && len > 0 && (hap.len - offset) >= len {
        let dest = unsafe { (hap.buffer as *mut ::core::ffi::c_uchar).offset(offset as isize) as *mut ::core::ffi::c_void };
        let dest_max = (hap.len - offset) as crate::types::size_t;
        let src = data as *const ::core::ffi::c_void;
        let count = len as crate::types::size_t;
        if unsafe { crate::compat::memcpy_s(dest, dest_max, src, count) } != crate::types::EOK as i32 {
            let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE as i32, crate::types::LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: memcpy_s fail\0".as_ptr() as *const _, b"HapPutData\0".as_ptr() as *const _, 50) };
        }
    }
}