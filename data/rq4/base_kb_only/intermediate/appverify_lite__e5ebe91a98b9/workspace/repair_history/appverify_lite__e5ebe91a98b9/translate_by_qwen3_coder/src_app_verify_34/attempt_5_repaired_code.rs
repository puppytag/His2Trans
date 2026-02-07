fn GetCertInfo(ctr: *const crate::types::mbedtls_x509_crt, binSignCert: *mut *mut crate::types::CertInfo) -> i32 {
    use crate::types::*;
    use crate::compat::memcpy_s;
    
    let certInfo: *mut CertInfo = unsafe { libc::malloc(core::mem::size_of::<CertInfo>()) as *mut CertInfo };
    if certInfo.is_null() {
        return V_ERR_MALLOC as i32;
    }
    
    let mut ret = crate::src_app_verify::CertInfoInit(certInfo);
    if ret != V_OK as i32 {
        ret = V_ERR_MEMSET as i32;
        unsafe {
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
        }
        return ret;
    }
    
    unsafe {
        (*certInfo).issuerLen = (*ctr).issuer_raw.len as i32;
        (*certInfo).subjectLen = (*ctr).subject_raw.len as i32;
        
        if (*certInfo).issuerLen == 0 || (*certInfo).issuerLen > (1024 * 1024) ||
           (*certInfo).subjectLen == 0 || (*certInfo).subjectLen > (1024 * 1024) {
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return V_ERR_MALLOC as i32;
        }
        
        (*certInfo).issuer = libc::malloc(((*certInfo).issuerLen + 1) as usize) as *mut core::ffi::c_char;
        if (*certInfo).issuer.is_null() {
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return V_ERR_MALLOC as i32;
        }
        *(*certInfo).issuer.offset((*certInfo).issuerLen as isize) = 0;
        ret = memcpy_s(
            (*certInfo).issuer as *mut core::ffi::c_void,
            (*certInfo).issuerLen as crate::types::size_t,
            (*ctr).issuer_raw.p as *const core::ffi::c_void,
            (*ctr).issuer_raw.len as crate::types::size_t,
        );
        if ret != 0 {
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return V_ERR_MEMCPY as i32;
        }
        
        (*certInfo).subject = libc::malloc(((*certInfo).subjectLen + 1) as usize) as *mut core::ffi::c_char;
        if (*certInfo).subject.is_null() {
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return V_ERR_MALLOC as i32;
        }
        *(*certInfo).subject.offset((*certInfo).subjectLen as isize) = 0;
        ret = memcpy_s(
            (*certInfo).subject as *mut core::ffi::c_void,
            (*certInfo).subjectLen as crate::types::size_t,
            (*ctr).subject_raw.p as *const core::ffi::c_void,
            (*ctr).subject_raw.len as crate::types::size_t,
        );
        if ret != 0 {
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return V_ERR_MEMCPY as i32;
        }
        
        (*certInfo).pkType = crate::compat::mbedtls_pk_get_type(&(*ctr).pk);
        let pk_buf = crate::src_app_verify::GetPkBuf(&(*ctr).pk, &mut (*certInfo).pkLen);
        (*certInfo).pkBuf = pk_buf as *mut core::ffi::c_char;
        if pk_buf.is_null() {
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return V_ERR as i32;
        }
        
        *binSignCert = certInfo;
    }
    V_OK as i32
}