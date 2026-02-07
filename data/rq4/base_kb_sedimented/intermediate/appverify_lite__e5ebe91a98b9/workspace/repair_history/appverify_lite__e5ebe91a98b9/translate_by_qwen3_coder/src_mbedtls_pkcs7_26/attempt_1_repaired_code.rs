fn ParseSignedDataCrl(p: *mut *mut u8, end: *const u8, crl: *mut crate::types::mbedtls_x509_crl) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;

    unsafe {
        rc = mbedtls_asn1_get_tag(
            p,
            end,
            &mut len,
            ((crate::types::MBEDTLS_ASN1_CONSTRUCTED | crate::types::MBEDTLS_ASN1_CONTEXT_SPECIFIC) + 1) as i32,
        );
    }
    
    if rc != 0 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Has no crl in signed data.\0".as_ptr() as *const ::core::ffi::c_char,
                b"ParseSignedDataCrl\0".as_ptr() as *const ::core::ffi::c_char,
                572i32,
            );
        }
        return crate::types::PKCS7_SUCC as i32;
    }
    
    unsafe {
        mbedtls_x509_crl_init(crl);
        rc = mbedtls_x509_crl_parse(crl, *p, len as crate::types::size_t);
        *p = (*p).add(len as usize);
    }
    
    rc
}