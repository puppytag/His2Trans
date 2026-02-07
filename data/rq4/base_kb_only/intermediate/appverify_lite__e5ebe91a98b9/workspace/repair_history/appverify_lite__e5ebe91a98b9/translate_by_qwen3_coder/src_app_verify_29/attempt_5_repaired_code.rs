fn CheckReleaseAppSign(binSignCert: *const crate::types::CertInfo, pf: *const crate::types::ProfileProf) -> i32 {
    unsafe {
        const APP_GALLERY: &[u8; 12] = b"app_gallery\0";
        let app_gallery_ptr: *const i8 = (APP_GALLERY.as_ptr)() as *const i8;
        
        if libc::strcmp((*pf).appDistType, app_gallery_ptr) == 0 {
            return crate::types::V_ERR as i32;
        }
        
        if libc::strlen((*pf).bundleInfo.releaseCert as *const i8) == 0 {
            return crate::types::V_ERR as i32;
        }
        
        let ret = LoadCertAndCmpDest((*pf).bundleInfo.releaseCert, binSignCert);
        if ret == crate::types::V_OK as i32 {
            return crate::types::V_OK as i32;
        }
        
        crate::types::V_ERR as i32
    }
}