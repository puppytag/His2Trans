fn ParseSignerSignature(p: *mut *mut ::core::ffi::c_uchar, end: *const ::core::ffi::c_uchar, signer: *mut crate::types::SignerInfo) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(p, end, &mut len, crate::types::MBEDTLS_ASN1_OCTET_STRING as i32) };
    if rc != 0 {
        return rc;
    }
    unsafe {
        (*signer).signature.tag = crate::types::MBEDTLS_ASN1_OCTET_STRING as i32;
        (*signer).signature.len = len;
        (*signer).signature.p = *p;
        *p = (*p).offset(len as isize);
    }
    return crate::types::PKCS7_SUCC as i32;
}