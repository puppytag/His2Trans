fn GetCertInfo(ctr: *const crate::types::mbedtls_x509_crt, binSignCert: *mut *mut crate::types::CertInfo) -> i32 {
    let cert_info_size = std::mem::size_of::<crate::types::CertInfo>();
    let cert_info = unsafe { libc::malloc(cert_info_size) } as *mut crate::types::CertInfo;
    if cert_info.is_null() {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: certInfo is null\0".as_ptr() as *const i8,
            "GetCertInfo\0".as_ptr() as *const i8,
            958,
        ) };
        return crate::types::V_ERR_MALLOC as i32;
    }
    let mut ret = crate::src_app_verify::CertInfoInit(cert_info);
    if ret != crate::types::V_OK as i32 {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: cert info init\0".as_ptr() as *const i8,
            "GetCertInfo\0".as_ptr() as *const i8,
            962,
        ) };
        ret = crate::types::V_ERR_MEMSET as i32;
        goto_exit;
    }
    unsafe {
        (*cert_info).issuerLen = (*ctr).issuer_raw.len as i32;
        (*cert_info).subjectLen = (*ctr).subject_raw.len as i32;
    }
    if unsafe { (*cert_info).issuerLen == 0 || (*cert_info).issuerLen > (1024 * 1024) ||
        (*cert_info).subjectLen == 0 || (*cert_info).subjectLen > (1024 * 1024) } {
        ret = crate::types::V_ERR_MALLOC as i32;
        goto_exit;
    }
    unsafe {
        (*cert_info).issuer = libc::malloc(((*cert_info).issuerLen + 1) as usize) as *mut i8;
    }
    if unsafe { (*cert_info).issuer.is_null() } {
        ret = crate::types::V_ERR_MALLOC as i32;
        goto_exit;
    }
    unsafe {
        *((*cert_info).issuer.offset((*cert_info).issuerLen as isize)) = 0;
        ret = crate::compat::memcpy_s(
            (*cert_info).issuer as *mut core::ffi::c_void,
            (*cert_info).issuerLen as crate::types::size_t,
            (*ctr).issuer_raw.p as *const core::ffi::c_void,
            (*ctr).issuer_raw.len,
        ) as i32;
    }
    if ret != 0 {
        ret = crate::types::V_ERR_MEMCPY as i32;
        goto_exit;
    }
    unsafe {
        (*cert_info).subject = libc::malloc(((*cert_info).subjectLen + 1) as usize) as *mut i8;
    }
    if unsafe { (*cert_info).subject.is_null() } {
        ret = crate::types::V_ERR_MALLOC as i32;
        goto_exit;
    }
    unsafe {
        *((*cert_info).subject.offset((*cert_info).subjectLen as isize)) = 0;
        ret = crate::compat::memcpy_s(
            (*cert_info).subject as *mut core::ffi::c_void,
            (*cert_info).subjectLen as crate::types::size_t,
            (*ctr).subject_raw.p as *const core::ffi::c_void,
            (*ctr).subject_raw.len,
        ) as i32;
    }
    if ret != 0 {
        ret = crate::types::V_ERR_MEMCPY as i32;
        goto_exit;
    }
    unsafe {
        (*cert_info).pkType = crate::compat::mbedtls_pk_get_type(&(*ctr).pk);
        (*cert_info).pkBuf = crate::src_app_verify::GetPkBuf(&(*ctr).pk, &mut (*cert_info).pkLen) as *mut i8;
    }
    if unsafe { (*cert_info).pkBuf.is_null() } {
        let _ = unsafe { crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_ERROR,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: get pk error\0".as_ptr() as *const i8,
            "GetCertInfo\0".as_ptr() as *const i8,
            998,
        ) };
        ret = crate::types::V_ERR as i32;
        goto_exit;
    }
    unsafe {
        *binSignCert = cert_info;
    }
    return crate::types::V_OK as i32;
    macro_rules! goto_exit {
        () => {
            crate::src_app_verify::FreeCertInfo(cert_info);
            if !cert_info.is_null() {
                unsafe { libc::free(cert_info as *mut core::ffi::c_void) };
            }
            return ret;
        };
    }
}