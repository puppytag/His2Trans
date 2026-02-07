fn GetCertInfo(ctr: *const crate::types::mbedtls_x509_crt, binSignCert: *mut *mut crate::types::CertInfo) -> i32 {
    let certInfo = unsafe { libc::malloc(std::mem::size_of::<crate::types::CertInfo>()) } as *mut crate::types::CertInfo;
    if certInfo.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: certInfo is null\0".as_ptr() as *const _, b"GetCertInfo\0".as_ptr() as *const _, 958) };
        return crate::types::V_ERR_MALLOC as i32;
    }
    let mut ret = crate::src_app_verify::CertInfoInit(certInfo);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: cert info init\0".as_ptr() as *const _, b"GetCertInfo\0".as_ptr() as *const _, 962) };
        ret = crate::types::V_ERR_MEMSET as i32;
        unsafe {
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut _);
        }
        return ret;
    }
    unsafe {
        (*certInfo).issuerLen = (*ctr).issuer_raw.len as i32;
        (*certInfo).subjectLen = (*ctr).subject_raw.len as i32;
    }
    if unsafe { (*certInfo).issuerLen == 0 || (*certInfo).issuerLen > (1024 * 1024) as i32 || (*certInfo).subjectLen == 0 || (*certInfo).subjectLen > (1024 * 1024) as i32 } {
        ret = crate::types::V_ERR_MALLOC as i32;
        unsafe {
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut _);
        }
        return ret;
    }
    unsafe {
        (*certInfo).issuer = libc::malloc(((*certInfo).issuerLen + 1) as usize) as *mut _;
        if (*certInfo).issuer.is_null() {
            ret = crate::types::V_ERR_MALLOC as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut _);
            return ret;
        }
        *((*certInfo).issuer.offset((*certInfo).issuerLen as isize)) = 0;
        ret = crate::compat::memcpy_s((*certInfo).issuer as *mut _, ((*certInfo).issuerLen as u64) as crate::types::size_t, (*ctr).issuer_raw.p, (*ctr).issuer_raw.len) as i32;
        if ret != 0 {
            ret = crate::types::V_ERR_MEMCPY as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut _);
            return ret;
        }
        (*certInfo).subject = libc::malloc(((*certInfo).subjectLen + 1) as usize) as *mut _;
        if (*certInfo).subject.is_null() {
            ret = crate::types::V_ERR_MALLOC as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut _);
            return ret;
        }
        *((*certInfo).subject.offset((*certInfo).subjectLen as isize)) = 0;
        ret = crate::compat::memcpy_s((*certInfo).subject as *mut _, ((*certInfo).subjectLen as u64) as crate::types::size_t, (*ctr).subject_raw.p, (*ctr).subject_raw.len) as i32;
        if ret != 0 {
            ret = crate::types::V_ERR_MEMCPY as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut _);
            return ret;
        }
        (*certInfo).pkType = crate::compat::mbedtls_pk_get_type(&(*ctr).pk);
        (*certInfo).pkBuf = crate::src_app_verify::GetPkBuf(&(*ctr).pk, &mut (*certInfo).pkLen) as *mut _;
        if (*certInfo).pkBuf.is_null() {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: get pk error\0".as_ptr() as *const _, b"GetCertInfo\0".as_ptr() as *const _, 998);
            ret = crate::types::V_ERR as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut _);
            return ret;
        }
        *binSignCert = certInfo;
    }
    crate::types::V_OK as i32
}