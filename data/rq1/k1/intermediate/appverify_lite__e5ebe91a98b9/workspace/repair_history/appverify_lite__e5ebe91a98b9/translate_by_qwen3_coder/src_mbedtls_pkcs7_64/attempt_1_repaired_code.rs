pub extern "C" fn PKCS7_GetContentData(pkcs7: *const crate::types::Pkcs7, data: *mut *mut ::core::ffi::c_uchar, dataLen: *mut crate::types::size_t) -> i32 {
    if pkcs7.is_null() || data.is_null() || dataLen.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    unsafe {
        let p = (*pkcs7).signedData.content.data.p;
        let len = (*pkcs7).signedData.content.data.len;
        let end = p.offset(len as isize);
        let mut octetLen: crate::types::size_t = 0;
        let mut p_mut = p;
        let rc = crate::compat::mbedtls_asn1_get_tag(&mut p_mut, end, &mut octetLen, crate::types::MBEDTLS_ASN1_OCTET_STRING as i32);
        if rc != 0 {
            return rc;
        }
        *data = p_mut;
        *dataLen = octetLen;
        crate::types::PKCS7_SUCC as i32
    }
}