pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    let mut hasContent: bool = false;
    let mut start: *mut u8;
    let mut end: *const u8;
    let mut bufLen = bufLen;

    if buf.is_null() || bufLen == 0 || pkcs7.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }

    unsafe {
        let _ = memset_s(
            pkcs7 as *mut ::core::ffi::c_void,
            std::mem::size_of::<crate::types::Pkcs7>() as crate::types::size_t,
            0,
            std::mem::size_of::<crate::types::Pkcs7>() as crate::types::size_t,
        );
    }

    start = buf as *mut u8;

    let mut format: std::ffi::c_char = 0;
    let mut pem: crate::types::mbedtls_pem_context = unsafe { std::mem::zeroed() };
    rc = crate::src_mbedtls_pkcs7::ParsePemFormatSignedData(buf, bufLen, &mut pem, &mut format);
    if rc != 0 {
        return rc;
    }
    if format == 1 {
        unsafe {
            start = pem.private_buf;
            bufLen = pem.private_buflen;
        }
    }

    end = unsafe { start.add(bufLen as usize) };

    rc = crate::src_mbedtls_pkcs7::LoadRootCert();
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::LoadSelfSignedCert();
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    rc = unsafe {
        crate::src_mbedtls_pkcs7::GetContentInfoType(
            &mut start,
            end,
            &mut (*pkcs7).contentTypeOid,
            &mut hasContent,
        )
    };
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    if !crate::src_mbedtls_pkcs7::IsSigedDataOid(pkcs7) || !hasContent {
        return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
    }

    rc = crate::src_mbedtls_pkcs7::GetContentLenOfContentInfo(&mut start, end, &mut len);
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    if unsafe { start.add(len as usize) } > end as *mut u8 {
        return crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
    }

    rc = unsafe {
        crate::src_mbedtls_pkcs7::ParseSignedData(start, len, &mut (*pkcs7).signedData)
    };
    if rc != crate::types::V_OK as i32 {
        return rc;
    }

    rc = unsafe { crate::src_mbedtls_pkcs7::ConstructSignerCerts(&mut (*pkcs7).signedData) };
    rc
}