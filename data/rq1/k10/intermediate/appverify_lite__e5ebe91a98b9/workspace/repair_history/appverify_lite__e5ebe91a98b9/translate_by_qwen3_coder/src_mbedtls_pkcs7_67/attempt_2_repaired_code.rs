pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    let mut hasContent: crate::types::c_bool = 0;
    let mut start: *mut u8 = std::ptr::null_mut();
    let mut end: *const u8 = std::ptr::null();
    if buf.is_null() || bufLen == 0 || pkcs7.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    unsafe {
        std::ptr::write_bytes(pkcs7 as *mut u8, 0, std::mem::size_of::<crate::types::Pkcs7>());
    }
    start = buf as *mut u8;
    let mut buf_len = bufLen;
    let mut format: ::core::ffi::c_char = 0;
    rc = crate::src_mbedtls_pkcs7::ParsePemFormatSignedData(buf, buf_len, unsafe { &mut (*pkcs7).pem }, &mut format);
    if rc != 0 {
        return rc;
    }
    if format == 1 {
        unsafe {
            start = (*pkcs7).pem.private_buf as *mut u8;
            buf_len = (*pkcs7).pem.private_buflen;
        }
    }
    unsafe {
        end = start.offset(buf_len as isize);
    }
    rc = crate::src_mbedtls_pkcs7::LoadRootCert();
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    rc = crate::src_mbedtls_pkcs7::LoadSelfSignedCert();
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    let mut p: *mut *mut ::core::ffi::c_uchar = &mut (start as *mut ::core::ffi::c_uchar);
    rc = crate::src_mbedtls_pkcs7::GetContentInfoType(p, end as *const ::core::ffi::c_uchar, unsafe { &mut (*pkcs7).contentTypeOid }, &mut hasContent);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    if !crate::src_mbedtls_pkcs7::IsSigedDataOid(pkcs7) || hasContent == 0 {
        return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
    }
    let mut p2: *mut *mut u8 = &mut start;
    rc = crate::src_mbedtls_pkcs7::GetContentLenOfContentInfo(p2, end, &mut len);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    unsafe {
        if start.offset(len as isize) > end {
            return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
        }
    }
    rc = crate::src_mbedtls_pkcs7::ParseSignedData(start, len as usize, unsafe { &mut (*pkcs7).signedData });
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    rc = crate::src_mbedtls_pkcs7::ConstructSignerCerts(unsafe { &mut (*pkcs7).signedData });
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    return rc;
}