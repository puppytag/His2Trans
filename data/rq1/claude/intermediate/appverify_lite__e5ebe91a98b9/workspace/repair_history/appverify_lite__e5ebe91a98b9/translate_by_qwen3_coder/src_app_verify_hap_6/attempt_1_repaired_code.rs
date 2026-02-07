fn UpdateSmallBlock(readLen: i32, fp: *const i32, mdCtx: *mut crate::types::mbedtls_md_context_t) -> i32 {
    let mut readLenLeft = readLen;
    while readLenLeft > 0 {
        let onceRead: i32 = if readLenLeft > (1024 * 64) { 1024 * 64 } else { readLenLeft };
        let onceBuf: *mut u8 = unsafe { libc::malloc(onceRead as usize) as *mut u8 };
        if onceBuf.is_null() {
            return crate::types::V_ERR as i32;
        }
        let len: isize = unsafe { libc::read(*fp, onceBuf as *mut core::ffi::c_void, (core::mem::size_of::<u8>() * onceRead as usize) as usize) };
        if len != onceRead as isize {
            unsafe { libc::free(onceBuf as *mut core::ffi::c_void) };
            return crate::types::V_ERR as i32;
        }
        let ret: i32 = unsafe { crate::compat::mbedtls_md_update(mdCtx, onceBuf, onceRead as u32) };
        unsafe { libc::free(onceBuf as *mut core::ffi::c_void) };
        if ret != crate::types::V_OK as i32 {
            return ret;
        }
        readLenLeft -= onceRead;
    }
    crate::types::V_OK as i32
}