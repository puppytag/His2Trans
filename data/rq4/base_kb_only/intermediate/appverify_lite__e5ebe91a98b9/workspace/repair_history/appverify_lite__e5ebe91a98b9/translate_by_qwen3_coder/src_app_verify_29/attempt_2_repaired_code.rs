fn CheckReleaseAppSign(binSignCert: *const crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        let app_gallery: &[u8] = b"app_gallery\0";
        let app_gallery_ptr = (app_gallery.as_ptr)() as *const i8;
        
        if libc::strcmp((*pf).appDistType, app_gallery_ptr) == 0 {
            return crate::types::V_ERR as i32;
        }
        
        if libc::strlen((*pf).bundleInfo.releaseCert as *const i8) == 0 {
            return crate::types::V_ERR as i32;
        }
        
        let ret = crate::src_app_verify::LoadCertAndCmpDest((*pf).bundleInfo.releaseCert, binSignCert);
        if ret == crate::types::V_OK as i32 {
            return crate::types::V_OK as i32;
        }
        
        crate::types::V_ERR as i32
    }
}