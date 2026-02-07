fn GetAppSignPublicKey(profile: *const crate::types::ProfileProf, pk: *mut crate::types::AppSignPk) -> i32 {
    let mut ret: i32;
    unsafe {
        if !(*profile).bundleInfo.releaseCert.is_null() &&
            libc::strlen((*profile).bundleInfo.releaseCert as *const std::ffi::c_char) != 0 {
            ret = crate::src_app_verify::ParseCertGetPk(
                (*profile).bundleInfo.releaseCert as *const std::ffi::c_char,
                pk,
            );
        } else {
            ret = crate::src_app_verify::ParseCertGetPk(
                (*profile).bundleInfo.devCert as *const std::ffi::c_char,
                pk,
            );
        }
    }
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            b"appverify\0".as_ptr() as *const std::ffi::c_char,
            b"[%s:%d]: GetSignCertpk failed, ret: %d\0".as_ptr() as *const std::ffi::c_char,
            b"GetAppSignPublicKey\0".as_ptr() as *const std::ffi::c_char,
            692,
            ret,
        );
        return crate::types::V_ERR_GET_CERT_PK as i32;
    }
    return crate::types::V_OK as i32;
}