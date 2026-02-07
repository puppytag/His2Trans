fn CmpCert(certA: *const crate::types::mbedtls_x509_crt, binSignCert: *const crate::types::CertInfo) -> i32 {
    unsafe {
        if certA.is_null() {
            return crate::types::V_ERR as i32;
        }
        if binSignCert.is_null() {
            return crate::types::V_ERR as i32;
        }

        let cert_a = &*certA;
        let bin_sign_cert = &*binSignCert;

        if cert_a.subject_raw.len as i32 != bin_sign_cert.subjectLen ||
            libc::memcmp(
                cert_a.subject_raw.p as *const core::ffi::c_void,
                bin_sign_cert.subject as *const core::ffi::c_void,
                cert_a.subject_raw.len as usize
            ) != 0 {
            return crate::types::V_ERR as i32;
        }

        if cert_a.issuer_raw.len as i32 != bin_sign_cert.issuerLen ||
            libc::memcmp(
                cert_a.issuer_raw.p as *const core::ffi::c_void,
                bin_sign_cert.issuer as *const core::ffi::c_void,
                cert_a.issuer_raw.len as usize
            ) != 0 {
            return crate::types::V_ERR as i32;
        }

        if crate::compat::mbedtls_pk_get_type(&cert_a.pk as *const crate::types::mbedtls_pk_context) != bin_sign_cert.pkType {
            return crate::types::V_ERR as i32;
        }

        let mut lenA: i32 = 0;
        let bufA = GetPkBuf(&cert_a.pk as *const crate::types::mbedtls_pk_context, &mut lenA as *mut i32);
        if bufA.is_null() {
            return crate::types::V_ERR as i32;
        }

        if lenA != bin_sign_cert.pkLen {
            libc::free(bufA as *mut core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }

        if libc::memcmp(
            bufA as *const core::ffi::c_void,
            bin_sign_cert.pkBuf as *const core::ffi::c_void,
            lenA as usize
        ) != 0 {
            libc::free(bufA as *mut core::ffi::c_void);
            return crate::types::V_ERR as i32;
        }

        libc::free(bufA as *mut core::ffi::c_void);
        crate::types::V_OK as i32
    }
}