函数: src_app_verify_40
文件: src_app_verify
尝试次数: 1/5
============================================================
翻译代码（修复前）:
============================================================
fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut binSignCert: *mut crate::types::CertInfo = std::ptr::null_mut();
    let mut certType: i32 = 0;
    let ret = crate::src_app_verify::VerifyBinSign(signInfo, fp, &mut binSignCert, &mut certType);
    if ret != 0 {
        let _ = crate::compat::HiLogPrint(3, 6, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify bin sign error\0".as_ptr() as *const _, b"VerifyIntegrity\0".as_ptr() as *const _, 1158);
        return ret;
    }
    let ret = crate::src_app_verify::VerfiyAppSourceGetProfile(fp, signInfo as *const _, certType, binSignCert, pf);
    if ret != 0 {
        let _ = crate::compat::HiLogPrint(3, 6, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: verify app source failed : %d\0".as_ptr() as *const _, b"VerifyIntegrity\0".as_ptr() as *const _, 1164, ret);
        crate::src_app_verify::FreeCertInfo(binSignCert);
        if !binSignCert.is_null() {
            unsafe { libc::free(binSignCert as *mut _) };
            binSignCert = std::ptr::null_mut();
        }
        return ret;
    }
    crate::src_app_verify::FreeCertInfo(binSignCert);
    if !binSignCert.is_null() {
        unsafe { libc::free(binSignCert as *mut _) };
        binSignCert = std::ptr::null_mut();
    }
    0
}