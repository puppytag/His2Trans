fn ParseSignedDataCerts(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, certs: *mut *mut crate::types::mbedtls_x509_crt) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    unsafe {
        rc = mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x80);
    }
    if rc != 0 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                "appverify\0".as_ptr() as *const ::core::ffi::c_char,
                "[%s:%d]: Has no certificates in signed data.\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return crate::types::PKCS7_SUCC as i32;
    }
    unsafe {
        *certs = libc::calloc(1, std::mem::size_of::<crate::types::mbedtls_x509_crt>()) as *mut crate::types::mbedtls_x509_crt;
        if (*certs).is_null() {
            return crate::types::PKCS7_MEMORY_EXHAUST as i32;
        }
        mbedtls_x509_crt_init(*certs);
        let certs_end = (*p).add(len as usize);
        let mut cnt = 0;
        while *p < certs_end {
            let mut one_cert_len: crate::types::size_t = 0;
            let seq_begin = *p;
            rc = mbedtls_asn1_get_tag(p, end, &mut one_cert_len, 0x20 | 0x10);
            if rc != 0 {
                return rc;
            }
            let seq_len = (*p).offset_from(seq_begin) as usize;
            if (one_cert_len as usize) + seq_len > certs_end.offset_from(seq_begin) as usize {
                return crate::types::PKCS7_PARSING_ERROR as i32;
            }
            rc = mbedtls_x509_crt_parse(*certs, seq_begin, (one_cert_len as usize + seq_len) as crate::types::size_t);
            if rc != 0 {
                return rc;
            }
            *p = (*p).add(one_cert_len as usize);
            cnt += 1;
        }
        let _ = HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            "appverify\0".as_ptr() as *const ::core::ffi::c_char,
            "[%s:%d]: Parse signed data certs success\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    rc
}