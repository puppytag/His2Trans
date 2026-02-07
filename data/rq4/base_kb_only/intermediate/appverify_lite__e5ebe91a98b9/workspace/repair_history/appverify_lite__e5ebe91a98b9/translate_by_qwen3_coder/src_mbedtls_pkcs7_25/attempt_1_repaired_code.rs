fn ParseSignedDataCerts(p: *mut *mut u8, end: *const u8, certs: *mut *mut crate::types::mbedtls_x509_crt) -> i32 {
    use crate::types::*;
    use crate::compat::*;
    
    let mut rc: i32;
    let mut len: size_t = 0;
    
    unsafe {
        rc = mbedtls_asn1_get_tag(p, end, &mut len, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_CONTEXT_SPECIFIC) as i32);
        if rc != 0 {
            let _ = HiLogPrint(
                LOG_CORE,
                LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: Has no certificates in signed data.\0".as_ptr() as *const i8,
                b"ParseSignedDataCerts\0".as_ptr() as *const i8,
                532i32,
            );
            return PKCS7_SUCC as i32;
        }
        
        *certs = mbedtls_calloc(1, core::mem::size_of::<mbedtls_x509_crt>() as size_t) as *mut mbedtls_x509_crt;
        if (*certs).is_null() {
            return PKCS7_MEMORY_EXHAUST as i32;
        }
        mbedtls_x509_crt_init(*certs);
        
        let certs_end: *mut u8 = (*p).add(len as usize);
        let mut _cnt: i32 = 0;
        
        while (*p) < certs_end {
            let mut one_cert_len: size_t = 0;
            let seq_begin: *mut u8 = *p;
            
            rc = mbedtls_asn1_get_tag(p, end, &mut one_cert_len, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE) as i32);
            if rc != 0 {
                return rc;
            }
            
            let p_offset = (*p).offset_from(seq_begin) as usize;
            let certs_end_offset = certs_end.offset_from(seq_begin) as usize;
            
            if (one_cert_len as usize) + p_offset > certs_end_offset {
                return PKCS7_PARSING_ERROR as i32;
            }
            
            let total_len = ((one_cert_len as usize) + p_offset) as size_t;
            rc = mbedtls_x509_crt_parse(*certs, seq_begin, total_len);
            if rc != 0 {
                return rc;
            }
            
            *p = (*p).add(one_cert_len as usize);
            _cnt += 1;
        }
        
        let _ = HiLogPrint(
            LOG_CORE,
            LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const i8,
            b"[%s:%d]: Parse signed data certs success\0".as_ptr() as *const i8,
            b"ParseSignedDataCerts\0".as_ptr() as *const i8,
            561i32,
        );
    }
    
    rc
}