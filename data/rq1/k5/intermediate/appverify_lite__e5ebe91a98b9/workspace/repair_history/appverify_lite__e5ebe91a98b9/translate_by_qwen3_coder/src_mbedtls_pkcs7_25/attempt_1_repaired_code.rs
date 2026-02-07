fn ParseSignedDataCerts(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, certs: *mut *mut crate::types::mbedtls_x509_crt) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x80) };
    if rc != 0 {
        let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Has no certificates in signed data.\0".as_ptr() as *const _, b"ParseSignedDataCerts\0".as_ptr() as *const _, 532) };
        return crate::types::PKCS7_SUCC as i32;
    }
    unsafe {
        *certs = crate::compat::mbedtls_calloc(1, ::core::mem::size_of::<crate::types::mbedtls_x509_crt>() as crate::types::size_t) as *mut crate::types::mbedtls_x509_crt;
    }
    if unsafe { (*certs).is_null() } {
        return crate::types::PKCS7_MEMORY_EXHAUST as i32;
    }
    unsafe {
        crate::compat::mbedtls_x509_crt_init(*certs);
    }
    let certs_end: *mut ::core::ffi::c_uchar = unsafe { (*p).wrapping_add(len as usize) };
    let mut cnt: i32 = 0;
    while unsafe { !(*p).is_null() && *p < certs_end } {
        let mut one_cert_len: crate::types::size_t = 0;
        let seq_begin: *mut ::core::ffi::c_uchar = unsafe { *p };
        rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut one_cert_len, 0x20 | 0x10) };
        if rc != 0 {
            return rc;
        }
        let seq_len = unsafe { (*p).offset_from(seq_begin) as usize };
        if (one_cert_len as usize) + seq_len > unsafe { certs_end.offset_from(seq_begin) as usize } {
            return crate::types::PKCS7_PARSING_ERROR as i32;
        }
        rc = unsafe { crate::compat::mbedtls_x509_crt_parse(*certs, seq_begin, (one_cert_len as usize + seq_len) as crate::types::size_t) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            *p = (*p).wrapping_add(one_cert_len as usize);
        }
        cnt += 1;
    }
    let _ = unsafe { crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Parse signed data certs success\0".as_ptr() as *const _, b"ParseSignedDataCerts\0".as_ptr() as *const _, 561) };
    return rc;
}