fn CalcDigest(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut u8, hashLen: *mut crate::types::size_t) -> i32 {
    let mut rc: i32;
    let mut input: *mut u8 = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;
    
    rc = crate::src_app_verify::CalcCmpContHash(pkcs7, signer, algType, hash, hashLen);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    
    rc = crate::src_mbedtls_pkcs7::PKCS7_GetSignerAuthAttr(signer, &mut input, &mut inputLen);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    
    rc = unsafe {
        crate::compat::mbedtls_md(
            crate::compat::mbedtls_md_info_from_type(algType),
            input,
            inputLen as u32,
            hash,
        )
    };
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    
    unsafe {
        *hashLen = crate::compat::mbedtls_md_get_size(
            crate::compat::mbedtls_md_info_from_type(algType)
        ) as crate::types::size_t;
    }
    
    crate::types::V_OK as i32
}