pub extern "C" fn APPVERI_FreeVerifyRst(verifyRst: *mut crate::types::VerifyResult) {
    if verifyRst.is_null() {
        return;
    }
    let _ = crate::compat::HiLogPrint(
        crate::types::LOG_CORE,
        crate::types::LOG_INFO,
        0xD001100,
        "appverify\0".as_ptr() as *const i8,
        "[%s:%d]: free verify rst data\0".as_ptr() as *const i8,
        __FUNCTION__,
        1256,
    );
    unsafe {
        crate::src_app_provision::ProfFreeData(&mut (*verifyRst).profile);
    }
}