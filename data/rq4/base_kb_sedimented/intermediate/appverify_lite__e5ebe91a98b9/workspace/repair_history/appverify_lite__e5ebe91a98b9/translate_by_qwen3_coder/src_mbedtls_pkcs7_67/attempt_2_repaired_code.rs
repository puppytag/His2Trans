pub extern "C" fn PKCS7_ParseSignedData(buf: *const ::core::ffi::c_uchar, bufLen: crate::types::size_t, pkcs7: *mut crate::types::Pkcs7) -> i32 {
    let mut rc: i32;
    let mut len: crate::types::size_t = 0;
    let mut hasContent: bool = false;
    let mut start: *mut u8 = std::ptr::null_mut();
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
    rc = crate::src_mbedtls_pkcs7::ParsePemFormatSignedData(buf, bufLen, unsafe { &mut (*pkcs7).pem }, &mut format);
    if rc != 0 {
        return rc;
    }
    if format == 1 {
        unsafe {
            start = (*pkcs7).pem.private_buf;
            bufLen = (*pkcs7).pem.private_buflen;
        }
    }

    end = unsafe { start.add(bufLen as usize) };

    rc = crate::src_mbedtls_pkcs7::LoadRootCert();
    if rc != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: rc not ok\0".as_ptr() as *const std::ffi::c_char,
                b"PKCS7_ParseSignedData\0".as_ptr() as *const std::ffi::c_char,
                1355i32,
            );
        }
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::LoadSelfSignedCert();
    if rc != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: rc not ok\0".as_ptr() as *const std::ffi::c_char,
                b"PKCS7_ParseSignedData\0".as_ptr() as *const std::ffi::c_char,
                1358i32,
            );
        }
        return rc;
    }

    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const std::ffi::c_char,
            b"[%s:%d]: Begin to parse pkcs#7 signed data\0".as_ptr() as *const std::ffi::c_char,
            b"PKCS7_ParseSignedData\0".as_ptr() as *const std::ffi::c_char,
            1360i32,
        );
    }

    rc = crate::src_mbedtls_pkcs7::GetContentInfoType(&mut start, end, unsafe { &mut (*pkcs7).contentTypeOid }, &mut hasContent);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: rc not ok\0".as_ptr() as *const std::ffi::c_char,
                b"PKCS7_ParseSignedData\0".as_ptr() as *const std::ffi::c_char,
                1363i32,
            );
        }
        return rc;
    }

    if !crate::src_mbedtls_pkcs7::IsSigedDataOid(pkcs7) || !hasContent {
        rc = crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: Input data is not pkcs#7 signed data format or has no content info\0".as_ptr() as *const std::ffi::c_char,
                b"PKCS7_ParseSignedData\0".as_ptr() as *const std::ffi::c_char,
                1366i32,
            );
        }
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::GetContentLenOfContentInfo(&mut start, end, &mut len);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: rc not ok\0".as_ptr() as *const std::ffi::c_char,
                b"PKCS7_ParseSignedData\0".as_ptr() as *const std::ffi::c_char,
                1370i32,
            );
        }
        return rc;
    }

    if unsafe { start.add(len as usize) } > end as *mut u8 {
        rc = crate::types::PKCS7_INVALID_CONTENT_TYPE_OR_NO_CONTENT as i32;
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: The length of input data is invalid\0".as_ptr() as *const std::ffi::c_char,
                b"PKCS7_ParseSignedData\0".as_ptr() as *const std::ffi::c_char,
                1373i32,
            );
        }
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedData(start, len, unsafe { &mut (*pkcs7).signedData });
    if rc != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: rc not ok\0".as_ptr() as *const std::ffi::c_char,
                b"PKCS7_ParseSignedData\0".as_ptr() as *const std::ffi::c_char,
                1377i32,
            );
        }
        return rc;
    }

    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const std::ffi::c_char,
            b"[%s:%d]: Parse pkcs#7 signed data success\0".as_ptr() as *const std::ffi::c_char,
            b"PKCS7_ParseSignedData\0".as_ptr() as *const std::ffi::c_char,
            1378i32,
        );
    }

    rc = crate::src_mbedtls_pkcs7::ConstructSignerCerts(unsafe { &mut (*pkcs7).signedData });
    if rc != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const std::ffi::c_char,
                b"[%s:%d]: rc not ok\0".as_ptr() as *const std::ffi::c_char,
                b"PKCS7_ParseSignedData\0".as_ptr() as *const std::ffi::c_char,
                1380i32,
            );
        }
        return rc;
    }

    return rc;
}