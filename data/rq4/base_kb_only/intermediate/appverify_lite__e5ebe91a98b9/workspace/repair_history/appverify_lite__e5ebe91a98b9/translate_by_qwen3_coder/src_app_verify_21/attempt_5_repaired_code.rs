fn GetPkBuf(pk: *const crate::types::mbedtls_pk_context, len: *mut i32) -> *mut u8 {
    let mut bufA: *mut u8 = std::ptr::null_mut();
    
    unsafe {
        let pk_type = mbedtls_pk_get_type(pk);
        
        if pk_type == crate::types::MBEDTLS_PK_RSA || pk_type == crate::types::MBEDTLS_PK_RSASSA_PSS {
            bufA = GetRsaPk(pk, len);
        } else if pk_type == crate::types::MBEDTLS_PK_ECDSA || pk_type == crate::types::MBEDTLS_PK_ECKEY {
            bufA = GetEcPk(pk, len);
        }
    }
    
    bufA
}