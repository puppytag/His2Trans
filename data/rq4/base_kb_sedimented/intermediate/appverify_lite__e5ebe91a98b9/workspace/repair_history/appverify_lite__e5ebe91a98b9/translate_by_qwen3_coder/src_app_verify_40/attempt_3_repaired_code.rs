函数: src_app_verify_40
文件: src_app_verify
尝试次数: 3/5
============================================================
修复后的代码:
============================================================
fn VerifyIntegrity(signInfo: *mut crate::types::SignatureInfo, fp: i32, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut binSignCert: *mut crate::types::CertInfo = std::ptr::null_mut();
    let mut certType: i32 = 0;
    
    let ret = VerifyBinSign(signInfo, fp, &mut binSignCert, &mut certType);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: verify bin sign error\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyIntegrity\0".as_ptr() as *const ::core::ffi::c_char,
                1158 as i32,
            );
        }
        return ret;
    }
    
    let ret = VerfiyAppSourceGetProfile(fp, signInfo as *const crate::types::SignatureInfo, certType, binSignCert, pf);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: verify app source failed : %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyIntegrity\0".as_ptr() as *const ::core::ffi::c_char,
                1164 as i32,
                ret,
            );
        }
        FreeCertInfo(binSignCert);
        if !binSignCert.is_null() {
            unsafe {
                libc::free(binSignCert as *mut ::core::ffi::c_void);
            }
        }
        return ret;
    }
    
    FreeCertInfo(binSignCert);
    if !binSignCert.is_null() {
        unsafe {
            libc::free(binSignCert as *mut ::core::ffi::c_void);
        }
    }
    
    crate::types::V_OK as i32
}