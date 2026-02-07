fn ProfInit(pf: *mut crate::types::ProfileProf) {
    if pf.is_null() {
        return;
    }
    let ret = unsafe {
        crate::compat::memset_s(
            pf as *mut ::core::ffi::c_void,
            std::mem::size_of::<crate::types::ProfileProf>() as crate::types::size_t,
            0,
            std::mem::size_of::<crate::types::ProfileProf>() as crate::types::size_t,
        )
    };
    if ret != crate::types::EOK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE as i32,
                crate::types::LOG_ERROR as i32,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: memset failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"ProfInit\0".as_ptr() as *const ::core::ffi::c_char,
                35,
            )
        };
        return;
    }
}