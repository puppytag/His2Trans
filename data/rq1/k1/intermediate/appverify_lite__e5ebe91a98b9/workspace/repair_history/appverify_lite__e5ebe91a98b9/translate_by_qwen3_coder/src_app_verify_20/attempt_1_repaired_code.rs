fn GetEcPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    unsafe {
        let ecCtx = {
            let pk_type = crate::compat::mbedtls_pk_get_type(pk);
            match pk_type {
                crate::types::MBEDTLS_PK_ECKEY |
                crate::types::MBEDTLS_PK_ECKEY_DH |
                crate::types::MBEDTLS_PK_ECDSA => (*pk).private_pk_ctx as *mut crate::types::mbedtls_ecp_keypair,
                _ => std::ptr::null_mut(),
            }
        };
        if ecCtx.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: get ec pk error\0".as_ptr() as *const i8,
                b"GetEcPk\0".as_ptr() as *const i8,
                597,
            );
            return std::ptr::null_mut();
        }
        let buf_size = (2 * ((521 + 7) / 8) + 1) as usize;
        let buf = libc::malloc(buf_size) as *mut u8;
        if buf.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: malloc error\0".as_ptr() as *const i8,
                b"GetEcPk\0".as_ptr() as *const i8,
                602,
            );
            return std::ptr::null_mut();
        }
        let ret = crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t);
        if ret != 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: memset error\0".as_ptr() as *const i8,
                b"GetEcPk\0".as_ptr() as *const i8,
                607,
            );
            libc::free(buf as *mut core::ffi::c_void);
            return std::ptr::null_mut();
        }
        let ret = crate::compat::mbedtls_ecp_point_write_binary(
            &(*ecCtx).private_grp,
            &(*ecCtx).private_Q,
            0,
            len as *mut crate::types::size_t,
            buf,
            buf_size as crate::types::size_t,
        );
        if ret != crate::types::V_OK as i32 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: get ecc pk key error\0".as_ptr() as *const i8,
                b"GetEcPk\0".as_ptr() as *const i8,
                614,
            );
            let _ = crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t);
            libc::free(buf as *mut core::ffi::c_void);
            return std::ptr::null_mut();
        }
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: GetEcPk *len %d\0".as_ptr() as *const i8,
            b"GetEcPk\0".as_ptr() as *const i8,
            619,
            *len,
        );
        if *len <= 0 || *len > buf_size as i32 {
            libc::free(buf as *mut core::ffi::c_void);
            return std::ptr::null_mut();
        }
        let pk_len = *len as usize;
        let pk_buf = libc::malloc(pk_len) as *mut u8;
        if pk_buf.is_null() {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: malloc error\0".as_ptr() as *const i8,
                b"GetEcPk\0".as_ptr() as *const i8,
                626,
            );
            let _ = crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t);
            libc::free(buf as *mut core::ffi::c_void);
            return std::ptr::null_mut();
        }
        let ret = crate::compat::memcpy_s(
            pk_buf as *mut core::ffi::c_void,
            pk_len as crate::types::size_t,
            buf as *const core::ffi::c_void,
            pk_len as crate::types::size_t,
        );
        if ret != 0 {
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: mem copy error: %d\0".as_ptr() as *const i8,
                b"GetEcPk\0".as_ptr() as *const i8,
                633,
                ret,
            );
            let _ = crate::compat::memset_s(buf as *mut core::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t);
            libc::free(buf as *mut core::ffi::c_void);
            libc::free(pk_buf as *mut core::ffi::c_void);
            return std::ptr::null_mut();
        }
        libc::free(buf as *mut core::ffi::c_void);
        pk_buf
    }
}