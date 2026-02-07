fn GetEcPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    use crate::compat::*;
    use crate::globals::*;
    use libc::{free, malloc};
    let ecCtx = unsafe {
        match crate::compat::mbedtls_pk_get_type(pk) {
            1 | 2 | 3 => (*pk).private_pk_ctx as *mut crate::types::mbedtls_ecp_keypair,
            _ => std::ptr::null_mut(),
        }
    };
    if ecCtx.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get ec pk error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 597) };
        return std::ptr::null_mut();
    }
    let buf_size = (2 * ((521 + 7) / 8) + 1) as usize;
    let buf = unsafe { malloc(buf_size) as *mut u8 };
    if buf.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 602) };
        return std::ptr::null_mut();
    }
    let ret = unsafe { memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: memset error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 607) };
        unsafe { free(buf as *mut std::ffi::c_void); }
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
    if ret != V_OK as i32 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get ecc pk key error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 614) };
        let _ = unsafe { memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { free(buf as *mut std::ffi::c_void); }
        return std::ptr::null_mut();
    }
    let len_val = unsafe { *len };
    let _ = unsafe { HiLogPrint(LOG_CORE, LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: GetEcPk *len %d\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 619, len_val) };
    if len_val <= 0 || len_val > buf_size as i32 {
        unsafe { free(buf as *mut std::ffi::c_void); }
        return std::ptr::null_mut();
    }
    let pkBuf = unsafe { malloc(len_val as usize) as *mut u8 };
    if pkBuf.is_null() {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 626) };
        let _ = unsafe { memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { free(buf as *mut std::ffi::c_void); }
        return std::ptr::null_mut();
    }
    let ret = unsafe { memcpy_s(pkBuf as *mut std::ffi::c_void, len_val as crate::types::size_t, buf as *const std::ffi::c_void, len_val as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { HiLogPrint(LOG_CORE, LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: mem copy error: %d\0".as_ptr() as *const i8, b"GetEcPk\0".as_ptr() as *const i8, 633, ret) };
        let _ = unsafe { memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { free(buf as *mut std::ffi::c_void); }
        unsafe { free(pkBuf as *mut std::ffi::c_void); }
        return std::ptr::null_mut();
    }
    unsafe { free(buf as *mut std::ffi::c_void); }
    pkBuf
}