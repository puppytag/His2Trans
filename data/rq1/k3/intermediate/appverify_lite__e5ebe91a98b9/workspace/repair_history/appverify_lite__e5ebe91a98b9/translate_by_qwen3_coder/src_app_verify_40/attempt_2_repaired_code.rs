函数: src_app_verify_40
文件: src_app_verify
尝试次数: 2/5
============================================================
修复后的代码:
============================================================
fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut binSignCert: *mut crate::types::CertInfo = std::ptr::null_mut();
    let mut certType: i32 = 0;
    let ret = crate::src_app_verify::VerifyBinSign(signInfo, fp, &mut binSignCert, &mut certType);
    if ret != crate::types::V_OK {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: verify bin sign error\0".as_ptr() as *const i8,
            __FUNCTION__,
            1158,
        );
        return ret;
    }
    let ret = crate::src_app_verify::VerfiyAppSourceGetProfile(fp, signInfo as *const _, certType, binSignCert, pf);
    if ret != crate::types::V_OK {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: verify app source failed : %d\0".as_ptr() as *const i8,
            __FUNCTION__,
            1164,
            ret,
        );
        crate::src_app_verify::FreeCertInfo(binSignCert);
        if !binSignCert.is_null() {
            unsafe {
                libc::free(binSignCert as *mut std::ffi::c_void);
            }
            binSignCert = std::ptr::null_mut();
        }
        return ret;
    }
    crate::src_app_verify::FreeCertInfo(binSignCert);
    if !binSignCert.is_null() {
        unsafe {
            libc::free(binSignCert as *mut std::ffi::c_void);
        }
        binSignCert = std::ptr::null_mut();
    }
    crate::types::V_OK as i32
}