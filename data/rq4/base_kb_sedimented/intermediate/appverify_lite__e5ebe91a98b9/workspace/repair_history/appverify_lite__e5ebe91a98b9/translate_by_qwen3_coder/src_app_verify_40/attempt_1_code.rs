函数: src_app_verify_40
文件: src_app_verify
尝试次数: 1/5
============================================================
翻译代码（修复前）:
============================================================
fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    use crate::types::*;
    
    let mut binSignCert: *mut CertInfo = std::ptr::null_mut();
    let mut certType: i32 = 0;
    
    let ret = crate::src_app_verify::VerifyBinSign(signInfo, fp, &mut binSignCert, &mut certType);
    if ret != V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: verify bin sign error\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyIntegrity\0".as_ptr() as *const ::core::ffi::c_char,
                1158 as i32,
            );
        }
        return ret;
    }
    
    let ret = crate::src_app_verify::VerfiyAppSourceGetProfile(fp, signInfo as *const SignatureInfo, certType, binSignCert, pf);
    if ret != V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: verify app source failed : %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyIntegrity\0".as_ptr() as *const ::core::ffi::c_char,
                1164 as i32,
                ret,
            );
        }
        crate::src_app_verify::FreeCertInfo(binSignCert);
        if !binSignCert.is_null() {
            unsafe {
                libc::free(binSignCert as *mut ::core::ffi::c_void);
            }
        }
        return ret;
    }
    
    crate::src_app_verify::FreeCertInfo(binSignCert);
    if !binSignCert.is_null() {
        unsafe {
            libc::free(binSignCert as *mut ::core::ffi::c_void);
        }
    }
    
    V_OK as i32
}