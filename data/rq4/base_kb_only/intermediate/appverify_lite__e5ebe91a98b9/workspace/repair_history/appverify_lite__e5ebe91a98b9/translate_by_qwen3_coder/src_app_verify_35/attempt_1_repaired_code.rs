函数: src_app_verify_35
文件: src_app_verify
尝试次数: 1/5
============================================================
修复后的代码:
============================================================
fn VerfiyAppSourceGetProfile(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut ret = crate::src_app_verify::VerifyProfGetContent(fp, signInfo, certType, pf);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: VerifyProfGetContent error: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerfiyAppSourceGetProfile\0".as_ptr() as *const ::core::ffi::c_char,
                1015i32,
                ret,
            )
        };
        return ret;
    }
    let _ = unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: verify prof get content success\0".as_ptr() as *const ::core::ffi::c_char,
            b"VerfiyAppSourceGetProfile\0".as_ptr() as *const ::core::ffi::c_char,
            1018i32,
        )
    };

    ret = crate::src_app_verify::CheckAppSignCertWithProfile(certType, binSignCert, pf);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: CheckAppSignCertWithProfile error: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerfiyAppSourceGetProfile\0".as_ptr() as *const ::core::ffi::c_char,
                1023i32,
                ret,
            )
        };
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR_VERFIY_PROF_CERT as i32;
    }

    unsafe {
        if !(*pf).bundleInfo.devCert.is_null() {
            libc::free((*pf).bundleInfo.devCert as *mut ::core::ffi::c_void);
            (*pf).bundleInfo.devCert = std::ptr::null_mut();
        }
        if !(*pf).bundleInfo.releaseCert.is_null() {
            libc::free((*pf).bundleInfo.releaseCert as *mut ::core::ffi::c_void);
            (*pf).bundleInfo.releaseCert = std::ptr::null_mut();
        }
    }

    let _ = unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: verfiy app source success\0".as_ptr() as *const ::core::ffi::c_char,
            b"VerfiyAppSourceGetProfile\0".as_ptr() as *const ::core::ffi::c_char,
            1032i32,
        )
    };
    crate::types::V_OK as i32
}