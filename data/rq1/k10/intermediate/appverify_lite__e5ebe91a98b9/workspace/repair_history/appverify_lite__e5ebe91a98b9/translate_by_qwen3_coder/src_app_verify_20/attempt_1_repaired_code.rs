fn GetEcPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    let ecCtx = unsafe {
        match crate::compat::mbedtls_pk_get_type(pk) {
            crate::types::MBEDTLS_PK_ECKEY |
            crate::types::MBEDTLS_PK_ECKEY_DH |
            crate::types::MBEDTLS_PK_ECDSA => (*pk).private_pk_ctx as *mut crate::types::mbedtls_ecp_keypair,
            _ => std::ptr::null_mut(),
        }
    };
    if ecCtx.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get ec pk error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 597) };
        return std::ptr::null_mut();
    }
    let buf_size = (2 * ((521 + 7) / 8) + 1) as usize;
    let buf = unsafe { libc::malloc(buf_size) } as *mut u8;
    if buf.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 602) };
        return std::ptr::null_mut();
    }
    let ret = unsafe { crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: memset error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 607) };
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    let ret = unsafe {
        crate::compat::mbedtls_ecp_point_write_binary(
            &(*ecCtx).private_grp,
            &(*ecCtx).private_Q,
            0,
            len as *mut crate::types::size_t,
            buf,
            buf_size as crate::types::size_t,
        )
    };
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get ecc pk key error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 614) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    let len_val = unsafe { *len };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: GetEcPk *len %d\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 619, len_val) };
    if len_val <= 0 || len_val > buf_size as i32 {
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    let pkBuf = unsafe { libc::malloc(len_val as usize) } as *mut u8;
    if pkBuf.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 626) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    let ret = unsafe { crate::compat::memcpy_s(pkBuf as *mut core::ffi::c_void, len_val as crate::types::size_t, buf as *const core::ffi::c_void, len_val as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: mem copy error: %d\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 633, ret) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut core::ffi::c_void) };
        unsafe { libc::free(pkBuf as *mut core::ffi::c_void) };
        return std::ptr::null_mut();
    }
    unsafe { libc::free(buf as *mut core::ffi::c_void) };
    pkBuf
}