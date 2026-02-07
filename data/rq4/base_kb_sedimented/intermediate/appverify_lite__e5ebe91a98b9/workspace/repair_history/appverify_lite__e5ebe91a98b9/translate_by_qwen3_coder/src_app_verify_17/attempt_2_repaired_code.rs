fn GetProfileSingerCertType(pkcs7Handle: *mut crate::types::Pkcs7, certType: *mut i32) -> i32 {
    let sri = crate::src_mbedtls_pkcs7::PKCS7_GetAllSignersResolvedInfo(pkcs7Handle as *const crate::types::Pkcs7);
    if sri.is_null() {
        return crate::types::V_ERR as i32;
    }
    
    let signer_ptr = unsafe { (*sri).signers };
    let ret = GetProfileCertTypeBySignInfo(signer_ptr, certType);
    if ret != crate::types::V_OK as i32 {
        crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
        return crate::types::V_ERR as i32;
    }
    
    crate::src_mbedtls_pkcs7::PKCS7_FreeAllSignersResolvedInfo(sri);
    crate::types::V_OK as i32
}