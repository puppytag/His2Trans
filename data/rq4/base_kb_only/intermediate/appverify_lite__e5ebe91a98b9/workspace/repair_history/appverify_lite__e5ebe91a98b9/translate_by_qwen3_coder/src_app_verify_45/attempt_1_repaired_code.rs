pub extern "C" fn APPVERI_FreeVerifyRst(verifyRst: *mut crate::types::VerifyResult) {
    if verifyRst.is_null() {
        return;
    }
    unsafe {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: free verify rst data\0".as_ptr() as *const ::core::ffi::c_char,
            b"APPVERI_FreeVerifyRst\0".as_ptr() as *const ::core::ffi::c_char,
            1256 as ::core::ffi::c_int,
        );
        crate::src_app_provision::ProfFreeData(&mut (*verifyRst).profile as *mut crate::types::ProfileProf);
    }
}