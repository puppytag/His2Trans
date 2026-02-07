fn GetCertInfo(ctr: *const crate::types::mbedtls_x509_crt, binSignCert: *mut *mut crate::types::CertInfo) -> i32 {
    unsafe {
        let certInfo = libc::malloc(core::mem::size_of::<crate::types::CertInfo>()) as *mut crate::types::CertInfo;
        if certInfo.is_null() {
            return crate::types::V_ERR_MALLOC as i32;
        }

        let mut ret = crate::src_app_verify::CertInfoInit(certInfo);
        if ret != crate::types::V_OK as i32 {
            ret = crate::types::V_ERR_MEMSET as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }

        (*certInfo).issuerLen = (*ctr).issuer_raw.len as i32;
        (*certInfo).subjectLen = (*ctr).subject_raw.len as i32;

        if (*certInfo).issuerLen == 0 || (*certInfo).issuerLen > (1024 * 1024) ||
           (*certInfo).subjectLen == 0 || (*certInfo).subjectLen > (1024 * 1024) {
            ret = crate::types::V_ERR_MALLOC as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }

        (*certInfo).issuer = libc::malloc(((*certInfo).issuerLen + 1) as usize) as *mut core::ffi::c_char;
        if (*certInfo).issuer.is_null() {
            ret = crate::types::V_ERR_MALLOC as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }
        *(*certInfo).issuer.offset((*certInfo).issuerLen as isize) = 0;
        ret = memcpy_s(
            (*certInfo).issuer as *mut core::ffi::c_void,
            (*certInfo).issuerLen as u32,
            (*ctr).issuer_raw.p as *const core::ffi::c_void,
            (*ctr).issuer_raw.len as u32
        ) as i32;
        if ret != 0 {
            ret = crate::types::V_ERR_MEMCPY as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }

        (*certInfo).subject = libc::malloc(((*certInfo).subjectLen + 1) as usize) as *mut core::ffi::c_char;
        if (*certInfo).subject.is_null() {
            ret = crate::types::V_ERR_MALLOC as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }
        *(*certInfo).subject.offset((*certInfo).subjectLen as isize) = 0;
        ret = memcpy_s(
            (*certInfo).subject as *mut core::ffi::c_void,
            (*certInfo).subjectLen as u32,
            (*ctr).subject_raw.p as *const core::ffi::c_void,
            (*ctr).subject_raw.len as u32
        ) as i32;
        if ret != 0 {
            ret = crate::types::V_ERR_MEMCPY as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }

        (*certInfo).pkType = mbedtls_pk_get_type(&(*ctr).pk as *const crate::types::mbedtls_pk_context);
        let pk_buf = crate::src_app_verify::GetPkBuf(&(*ctr).pk as *const crate::types::mbedtls_pk_context, &mut (*certInfo).pkLen as *mut i32);
        (*certInfo).pkBuf = pk_buf as *mut core::ffi::c_char;
        if (*certInfo).pkBuf.is_null() {
            ret = crate::types::V_ERR as i32;
            crate::src_app_verify::FreeCertInfo(certInfo);
            libc::free(certInfo as *mut core::ffi::c_void);
            return ret;
        }

        *binSignCert = certInfo;
        crate::types::V_OK as i32
    }
}