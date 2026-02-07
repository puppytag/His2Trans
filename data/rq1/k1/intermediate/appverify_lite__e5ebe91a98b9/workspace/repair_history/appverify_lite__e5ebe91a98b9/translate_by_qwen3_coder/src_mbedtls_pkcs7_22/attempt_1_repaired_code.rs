fn ParseSignedDataDigestAlgs(p: *mut *mut core::ffi::c_uchar, end: *const core::ffi::c_uchar, algIds: *mut crate::types::DigestAlgId) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    unsafe {
        rc = mbedtls_asn1_get_tag(p, end, &mut len, 0x20 | 0x11);
    }
    if rc != 0 {
        return rc;
    }
    let mut end_local: *const core::ffi::c_uchar = unsafe { (*p).add(len as usize) };
    let mut id: *mut crate::types::DigestAlgId = algIds;
    unsafe {
        while *p < end_local {
            let mut params = crate::types::mbedtls_asn1_buf {
                tag: 0,
                len: 0,
                p: std::ptr::null_mut(),
            };
            rc = mbedtls_asn1_get_alg(p, end_local, &mut (*id).algBuf, &mut params);
            if rc != 0 {
                return rc;
            }
            if crate::src_mbedtls_pkcs7::InvalidDigestAlg(&(*id).algBuf as *const crate::types::mbedtls_asn1_buf) {
                return crate::types::PKCS7_INVALID_DIGEST_ALG as i32;
            }
            if *p < end_local {
                (*id).next = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<crate::types::DigestAlgId>() as crate::types::size_t) as *mut crate::types::DigestAlgId;
                if (*id).next.is_null() {
                    return crate::types::PKCS7_MEMORY_EXHAUST as i32;
                }
                id = (*id).next;
            }
        }
    }
    crate::types::PKCS7_SUCC as i32
}