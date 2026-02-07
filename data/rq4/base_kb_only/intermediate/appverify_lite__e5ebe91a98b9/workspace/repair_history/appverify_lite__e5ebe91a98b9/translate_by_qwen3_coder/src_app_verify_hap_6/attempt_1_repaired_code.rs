fn UpdateSmallBlock(readLen: i32, fp: i32, mdCtx: *mut crate::types::mbedtls_md_context_t) -> i32 {
    let mut readLenLeft = readLen;
    while readLenLeft > 0 {
        let onceRead = if readLenLeft > (1024 * 64) { 1024 * 64 } else { readLenLeft };
        let mut onceBuf = unsafe { libc::malloc(onceRead as usize) as *mut u8 };
        if onceBuf.is_null() {
            unsafe {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: \"onceBuf\" is null\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    193i32,
                );
            }
            return crate::types::V_ERR as i32;
        }
        let len = unsafe { libc::read(fp, onceBuf as *mut core::ffi::c_void, (core::mem::size_of::<i8>() * onceRead as usize) as usize) } as i32;
        if len != onceRead {
            unsafe {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: \"fread err: %d, %d\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    196i32,
                    len,
                    onceRead,
                );
            }
            if !onceBuf.is_null() {
                unsafe { libc::free(onceBuf as *mut core::ffi::c_void) };
                onceBuf = std::ptr::null_mut();
            }
            return crate::types::V_ERR as i32;
        }
        let ret = unsafe { mbedtls_md_update(mdCtx, onceBuf, onceRead as u32) };
        if !onceBuf.is_null() {
            unsafe { libc::free(onceBuf as *mut core::ffi::c_void) };
            onceBuf = std::ptr::null_mut();
        }
        if ret != crate::types::V_OK as i32 {
            unsafe {
                HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: \"ret\" not ok\0".as_ptr() as *const i8,
                    b"UpdateSmallBlock\0".as_ptr() as *const i8,
                    202i32,
                );
            }
            return ret;
        }
        readLenLeft -= onceRead;
    }
    crate::types::V_OK as i32
}