fn CertInfoInit(certInfo: *mut crate::types::CertInfo) -> i32 {
    if certInfo.is_null() {
        return 0;
    }
    let size = std::mem::size_of::<crate::types::CertInfo>();
    let ret = unsafe { crate::compat::memset_s(certInfo as *mut std::ffi::c_void, size as crate::types::size_t, 0, size as crate::types::size_t) };
    if ret != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, "appverify\0".as_ptr() as *const std::ffi::c_char, "[%s:%d]: memset error\0".as_ptr() as *const std::ffi::c_char, __FUNCTION__, 928) };
    }
    ret
}