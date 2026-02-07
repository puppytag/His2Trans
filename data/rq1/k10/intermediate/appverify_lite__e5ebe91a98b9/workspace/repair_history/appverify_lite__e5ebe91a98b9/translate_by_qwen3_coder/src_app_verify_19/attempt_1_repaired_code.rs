fn GetRsaPk(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *const u8 {
    let buf_size = 1024 * 2 + 20;
    let buf = unsafe { libc::malloc(buf_size as usize) as *mut u8 };
    if buf.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 554) };
        return std::ptr::null();
    }
    let ret = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: memset error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 559) };
        unsafe { libc::free(buf as *mut std::ffi::c_void); }
        return std::ptr::null();
    }
    let c = unsafe { buf.add(buf_size as usize) };
    let pk_len = unsafe { crate::compat::mbedtls_pk_write_pubkey(&c as *const *mut u8 as *mut *mut u8, buf, pk) };
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: GetRsaPk pkLen %d\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 565, pk_len) };
    if pk_len < 0 || pk_len > buf_size {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: get pk buf error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 567) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut std::ffi::c_void); }
        return std::ptr::null();
    }
    let pk_buf = unsafe { libc::malloc(pk_len as usize) as *mut u8 };
    if pk_buf.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: malloc error\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 574) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut std::ffi::c_void); }
        return std::ptr::null();
    }
    let ret = unsafe { crate::compat::memcpy_s(pk_buf as *mut std::ffi::c_void, pk_len as crate::types::size_t, c as *const std::ffi::c_void, pk_len as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: mem copy error: %d\0".as_ptr() as *const i8, b"GetRsaPk\0".as_ptr() as *const i8, 581, ret) };
        let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
        unsafe { libc::free(buf as *mut std::ffi::c_void); }
        unsafe { libc::free(pk_buf as *mut std::ffi::c_void); }
        return std::ptr::null();
    }
    unsafe { *len = pk_len; }
    let _ = unsafe { crate::compat::memset_s(buf as *mut std::ffi::c_void, buf_size as crate::types::size_t, 0, buf_size as crate::types::size_t) };
    unsafe { libc::free(buf as *mut std::ffi::c_void); }
    pk_buf as *const u8
}