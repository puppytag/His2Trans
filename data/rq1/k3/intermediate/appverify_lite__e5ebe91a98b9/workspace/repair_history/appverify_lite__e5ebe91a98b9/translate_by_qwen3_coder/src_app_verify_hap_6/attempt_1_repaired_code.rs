fn UpdateSmallBlock(readLen: i32, fp: *const i32, mdCtx: *mut crate::types::mbedtls_md_context_t) -> i32 {
    let mut readLenLeft = readLen;
    while readLenLeft > 0 {
        let onceRead = if readLenLeft > 65536 { 65536 } else { readLenLeft };
        let onceBuf = unsafe { libc::malloc(onceRead as usize) as *mut u8 };
        if onceBuf.is_null() {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: onceBuf is null\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    193,
                );
            }
            return crate::types::V_ERR as i32;
        }
        let len = unsafe { libc::read(*fp as i32, onceBuf as *mut core::ffi::c_void, onceRead as usize) };
        if len != onceRead as isize {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: fread err: %d, %d\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    196,
                    len as i32,
                    onceRead,
                );
                libc::free(onceBuf as *mut core::ffi::c_void);
            }
            return crate::types::V_ERR as i32;
        }
        let ret = unsafe {
            crate::compat::mbedtls_md_update(
                mdCtx,
                onceBuf as *const u8,
                onceRead as crate::types::size_t,
            )
        };
        unsafe { libc::free(onceBuf as *mut core::ffi::c_void) };
        if ret != crate::types::V_OK as i32 {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: ret not ok\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    202,
                );
            }
            return ret;
        }
        readLenLeft -= onceRead;
    }
    crate::types::V_OK as i32
}