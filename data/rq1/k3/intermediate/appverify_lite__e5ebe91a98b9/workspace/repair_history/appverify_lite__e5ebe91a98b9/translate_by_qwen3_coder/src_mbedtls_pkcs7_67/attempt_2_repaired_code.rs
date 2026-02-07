pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32 {
    let mut rc: i32 = 0;
    let mut len: crate::types::size_t = 0;
    let mut hasContent: crate::types::c_bool = 0;
    let mut start: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut end: *const ::core::ffi::c_uchar = std::ptr::null();
    if buf.is_null() || bufLen == 0 || pkcs7.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    unsafe {
        std::ptr::write_bytes(pkcs7 as *mut u8, 0, std::mem::size_of::<crate::types::Pkcs7>());
    }
    start = buf as *mut ::core::ffi::c_uchar;
    let mut format: ::core::ffi::c_char = 0;
    rc = crate::src_mbedtls_pkcs7::ParsePemFormatSignedData(buf, bufLen, std::ptr::null_mut(), &mut format as *mut ::core::ffi::c_char);
    if rc != 0 {
        return rc;
    }
    if format == 1 {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    unsafe {
        end = start.offset(bufLen as isize) as *const ::core::ffi::c_uchar;
    }
    rc = crate::src_mbedtls_pkcs7::LoadRootCert();
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    rc = crate::src_mbedtls_pkcs7::LoadSelfSignedCert();
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    rc = crate::src_mbedtls_pkcs7::GetContentInfoType(&mut start as *mut *mut ::core::ffi::c_uchar, end, unsafe { &mut (*pkcs7).contentTypeOid } as *mut crate::types::mbedtls_asn1_buf, &mut hasContent as *mut crate::types::c_bool);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    if !crate::src_mbedtls_pkcs7::IsSigedDataOid(pkcs7) || hasContent == 0 {
        rc = crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
        return rc;
    }
    rc = crate::src_mbedtls_pkcs7::GetContentLenOfContentInfo(&mut start as *mut *mut u8, end as *const u8, &mut len as *mut crate::types::size_t);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    unsafe {
        if start.offset(len as isize) > end as *mut ::core::ffi::c_uchar {
            rc = crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
            return rc;
        }
    }
    rc = crate::src_mbedtls_pkcs7::ParseSignedData(start as *mut u8, len as usize, unsafe { &mut (*pkcs7).signedData } as *mut crate::types::SignedData);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    rc = crate::src_mbedtls_pkcs7::ConstructSignerCerts(unsafe { &mut (*pkcs7).signedData } as *mut crate::types::SignedData);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }
    return rc;
}