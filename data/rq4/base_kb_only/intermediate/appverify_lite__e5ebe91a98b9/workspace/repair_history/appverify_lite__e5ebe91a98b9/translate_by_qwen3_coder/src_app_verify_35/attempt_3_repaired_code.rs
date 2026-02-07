函数: src_app_verify_35
文件: src_app_verify
尝试次数: 3/5
============================================================
修复后的代码:
============================================================
fn VerfiyAppSourceGetProfile(fp: i32, signInfo: *const crate::types::SignatureInfo, certType: i32, binSignCert: *mut crate::types::CertInfo, pf: *mut crate::types::ProfileProf) -> i32 {
    let mut ret = crate::src_app_verify::VerifyProfGetContent(fp, signInfo, certType, pf);
    if ret != crate::types::V_OK as i32 {
        return ret;
    }

    ret = crate::src_app_verify::CheckAppSignCertWithProfile(certType, binSignCert, pf);
    if ret != crate::types::V_OK as i32 {
        crate::src_app_provision::ProfFreeData(pf);
        return crate::types::V_ERR_VERFIY_PROF_CERT as i32;
    }

    unsafe {
        let dev_cert = (*pf).bundleInfo.devCert;
        if !dev_cert.is_null() {
            libc::free(dev_cert as *mut ::core::ffi::c_void);
            (*pf).bundleInfo.devCert = std::ptr::null_mut();
        }
        let release_cert = (*pf).bundleInfo.releaseCert;
        if !release_cert.is_null() {
            libc::free(release_cert as *mut ::core::ffi::c_void);
            (*pf).bundleInfo.releaseCert = std::ptr::null_mut();
        }
    }

    crate::types::V_OK as i32
}