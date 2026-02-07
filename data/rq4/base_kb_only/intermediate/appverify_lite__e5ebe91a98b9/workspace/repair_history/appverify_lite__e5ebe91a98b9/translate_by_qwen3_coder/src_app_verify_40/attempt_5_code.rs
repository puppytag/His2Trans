函数: src_app_verify_40
文件: src_app_verify
尝试次数: 5/5
============================================================
翻译代码（修复前）:
============================================================
fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut binSignCert: *mut crate::types::CertInfo = std::ptr::null_mut();
    let mut certType: i32 = 0;

    let ret = VerifyBinSign(signInfo, fp, &mut binSignCert, &mut certType);
    if ret != crate::types::V_OK as i32 {
        return ret;
    }

    let ret2 = VerfiyAppSourceGetProfile(fp, signInfo as *const crate::types::SignatureInfo, certType, binSignCert, pf);
    if ret2 != crate::types::V_OK as i32 {
        FreeCertInfo(binSignCert);
        if !binSignCert.is_null() {
            unsafe { libc::free(binSignCert as *mut ::core::ffi::c_void) };
        }
        return ret2;
    }
    FreeCertInfo(binSignCert);
    if !binSignCert.is_null() {
        unsafe { libc::free(binSignCert as *mut ::core::ffi::c_void) };
    }
    crate::types::V_OK as i32
}