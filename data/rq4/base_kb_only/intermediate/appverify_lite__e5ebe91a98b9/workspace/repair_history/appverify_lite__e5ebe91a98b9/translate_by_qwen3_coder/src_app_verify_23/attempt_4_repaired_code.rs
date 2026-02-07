fn GetAppSignPublicKey(profile: *const crate::types::ProfileProf, pk: *mut crate::types::AppSignPk) -> i32 {
    let ret: i32;

    unsafe {
        let release_cert = (*profile).bundleInfo.releaseCert;
        if !release_cert.is_null() && libc::strlen(release_cert as *const i8) != 0 {
            ret = ParseCertGetPk(release_cert as *const core::ffi::c_char, pk);
        } else {
            let dev_cert = (*profile).bundleInfo.devCert;
            ret = ParseCertGetPk(dev_cert as *const core::ffi::c_char, pk);
        }
        
        if ret != crate::types::V_OK as i32 {
            return crate::types::V_ERR_GET_CERT_PK as i32;
        }
        crate::types::V_OK as i32
    }
}