fn UpdateSmallBlock(readLen: i32, fp: *const i32, mdCtx: *mut crate::types::mbedtls_md_context_t) -> i32 {
    let mut readLenLeft = readLen;
    while readLenLeft > 0 {
        let onceRead = if readLenLeft > crate::types::ONCE_READ_LEN as i32 {
            crate::types::ONCE_READ_LEN as i32
        } else {
            readLenLeft
        };
        let onceBuf = unsafe { libc::malloc(onceRead as usize) as *mut u8 };
        if onceBuf.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: onceBuf is null\0".as_ptr() as *const i8,
                __FUNCTION__,
                193,
            );
            return crate::types::V_ERR as i32;
        }
        let len = unsafe { libc::read(*fp as i32, onceBuf as *mut core::ffi::c_void, onceRead as usize) };
        if len != onceRead as isize {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: fread err: %d, %d\0".as_ptr() as *const i8,
                __FUNCTION__,
                196,
                len as i32,
                onceRead,
            );
            if !onceBuf.is_null() {
                unsafe { libc::free(onceBuf as *mut core::ffi::c_void) };
            }
            return crate::types::V_ERR as i32;
        }
        let ret = unsafe { crate::compat::mbedtls_md_update(mdCtx, onceBuf as *const u8, onceRead as crate::types::size_t) };
        if !onceBuf.is_null() {
            unsafe { libc::free(onceBuf as *mut core::ffi::c_void) };
        }
        if ret != crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const i8,
                "[%s:%d]: ret not ok\0".as_ptr() as *const i8,
                __FUNCTION__,
                202,
            );
            return ret;
        }
        readLenLeft -= onceRead;
    }
    crate::types::V_OK as i32
}