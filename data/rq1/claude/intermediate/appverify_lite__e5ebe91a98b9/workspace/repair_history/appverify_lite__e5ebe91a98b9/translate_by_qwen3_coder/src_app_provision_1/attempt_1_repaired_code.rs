fn ProfInit(pf: *mut crate::types::ProfileProf) {
    let ret: crate::types::errno_t = unsafe {
        crate::compat::memset_s(
            pf as *mut ::core::ffi::c_void,
            std::mem::size_of::<crate::types::ProfileProf>() as u32,
            0,
            std::mem::size_of::<crate::types::ProfileProf>() as u32,
        )
    };
    if ret != crate::types::EOK as crate::types::errno_t {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: memset failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"ProfInit\0".as_ptr() as *const ::core::ffi::c_char,
                35i32,
            );
        }
        return;
    }
    return;
}