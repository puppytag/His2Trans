fn ParseSignedDataDigestAlgs(p: *mut *mut u8, end: *const u8, algIds: *mut crate::types::DigestAlgId) -> i32 {
    use crate::types::*;
    
    let mut len: size_t = 0;
    
    let rc = unsafe {
        crate::compat::mbedtls_asn1_get_tag(
            p,
            end,
            &mut len,
            (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SET) as i32
        )
    };
    if rc != 0 {
        return rc;
    }
    
    let end_local: *const u8 = unsafe { (*p).add(len as usize) };
    
    let mut id: *mut DigestAlgId = algIds;
    while unsafe { ((*p) as usize) < (end_local as usize) } {
        let mut params = mbedtls_asn1_buf {
            tag: 0,
            len: 0,
            p: std::ptr::null_mut(),
        };
        
        let rc = unsafe {
            crate::compat::mbedtls_asn1_get_alg(
                p,
                end_local,
                &mut (*id).algBuf,
                &mut params
            )
        };
        if rc != 0 {
            return rc;
        }
        
        if crate::src_mbedtls_pkcs7::InvalidDigestAlg(unsafe { &(*id).algBuf }) {
            return PKCS7_INVALID_DIGEST_ALG as i32;
        }
        
        if unsafe { ((*p) as usize) < (end_local as usize) } {
            let next_ptr: *mut ::core::ffi::c_void = crate::src_mbedtls_pkcs7::Pkcs7Calloc(
                1 as size_t,
                std::mem::size_of::<DigestAlgId>() as size_t
            );
            if next_ptr.is_null() {
                return PKCS7_MEMORY_EXHAUST as i32;
            }
            unsafe {
                (*id).next = next_ptr as *mut DigestAlgId;
                id = (*id).next;
            }
        }
    }
    
    PKCS7_SUCC as i32
}