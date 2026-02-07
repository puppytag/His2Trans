fn CheckDebugAppSign(binSignCert: *mut crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let dev_cert = (*pf).bundleInfo.devCert;
        if libc::strlen(dev_cert as *const i8) == 0 {
            return crate::types::V_ERR as i32;
        }
        
        let ret = crate::src_app_verify::LoadCertAndCmpDest(dev_cert, binSignCert as *const crate::types::CertInfo);
        if ret == crate::types::V_OK as i32 {
            return crate::types::V_OK as i32;
        }
        
        let release_cert = (*pf).bundleInfo.releaseCert;
        if libc::strlen(release_cert as *const i8) != 0 {
            let ret2 = crate::src_app_verify::LoadCertAndCmpDest(release_cert, binSignCert as *const crate::types::CertInfo);
            if ret2 == crate::types::V_OK as i32 {
                return crate::types::V_OK as i32;
            }
        }
        
        crate::types::V_ERR as i32
    }
}