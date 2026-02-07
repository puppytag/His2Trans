fn ParseSignedDataSignerInfos(p: *mut *mut u8, end: *const u8, signers: *mut crate::types::SignerInfo) -> i32 {
    use crate::types::*;
    
    let mut rc: i32;
    let mut len: size_t = 0;
    
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SET) as i32) };
    if rc != 0 || len == 0 {
        return PKCS7_HAS_NO_SIGNER_INFO as i32;
    }
    
    let mut end = unsafe { (*p).add(len as usize) };
    let mut signers = signers;
    
    while unsafe { (*p) < end } {
        let mut oneSignerLen: size_t = 0;
        
        rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut oneSignerLen, (MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE) as i32) };
        if rc != PKCS7_SUCC as i32 {
            return rc;
        }
        
        let oneSignerEnd = unsafe { (*p).add(oneSignerLen as usize) };
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerVersion(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerIssuerAndSerialNum(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerDigestAlg(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerAuthAttr(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerEncAlg(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerSignature(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            return rc;
        }
        
        rc = crate::src_mbedtls_pkcs7::ParseSignerUnAuthAttr(p, oneSignerEnd, signers);
        if rc != PKCS7_SUCC as i32 {
            return rc;
        }
        
        if unsafe { (*p) < end } {
            let next_signer = crate::src_mbedtls_pkcs7::Pkcs7Calloc(1, std::mem::size_of::<SignerInfo>() as size_t) as *mut SignerInfo;
            if next_signer.is_null() {
                return PKCS7_MEMORY_EXHAUST as i32;
            }
            unsafe { (*signers).next = next_signer };
            signers = next_signer;
        }
    }
    rc
}