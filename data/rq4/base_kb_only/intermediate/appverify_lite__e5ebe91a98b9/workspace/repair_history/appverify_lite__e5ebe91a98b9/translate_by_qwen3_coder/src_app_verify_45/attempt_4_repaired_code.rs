pub extern "C" fn APPVERI_FreeVerifyRst(verifyRst: *mut crate::types::VerifyResult) {
    if verifyRst.is_null() {
        return;
    }
    unsafe {
        crate::src_app_provision::ProfFreeData(&mut (*verifyRst).profile);
    }
}