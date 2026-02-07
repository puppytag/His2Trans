函数: src_app_verify_35
文件: src_app_verify
尝试次数: 1/5
============================================================
翻译代码（修复前）:
============================================================
fn VerfiyAppSourceGetProfile(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut ret = crate::src_app_verify::VerifyProfGetContent(fp, signInfo, certType, pf);
    if ret != crate::types::V_OK {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: VerifyProfGetContent error: %d\0".as_ptr() as *const i8, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const i8, 1015, ret);
        return ret;
    }
    let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: verify prof get content success\0".as_ptr() as *const i8, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const i8, 1018);

    ret = crate::src_app_verify::CheckAppSignCertWithProfile(certType, binSignCert as *const crate::types::CertInfo, pf as *const crate::types::ProfileProf);
    if ret != crate::types::V_OK {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: CheckAppSignCertWithProfile error: %d\0".as_ptr() as *const i8, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const i8, 1023, ret);
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR_VERFIY_PROF_CERT as i32;
    }

    unsafe {
        if !(*pf).bundleInfo.devCert.is_null() {
            libc::free((*pf).bundleInfo.devCert as *mut libc::c_void);
            (*pf).bundleInfo.devCert = std::ptr::null_mut();
        }
        if !(*pf).bundleInfo.releaseCert.is_null() {
            libc::free((*pf).bundleInfo.releaseCert as *mut libc::c_void);
            (*pf).bundleInfo.releaseCert = std::ptr::null_mut();
        }
    }

    let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const i8, b"[%s:%d]: verfiy app source success\0".as_ptr() as *const i8, b"VerfiyAppSourceGetProfile\0".as_ptr() as *const i8, 1032);
    return crate::types::V_OK as i32;
}