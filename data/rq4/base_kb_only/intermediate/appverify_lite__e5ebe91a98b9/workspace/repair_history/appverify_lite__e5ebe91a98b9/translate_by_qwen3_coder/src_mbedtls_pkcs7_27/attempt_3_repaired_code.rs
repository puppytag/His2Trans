fn ParseSignedData(buf: *mut u8, bufLen: crate::types::size_t, signedData: *mut crate::types::SignedData) -> i32 {
    let mut p: *mut u8 = buf;
    let end: *const u8 = unsafe { buf.offset(bufLen as isize) };
    let mut len: crate::types::size_t = 0;
    let mut rc: i32;

    rc = unsafe { mbedtls_asn1_get_tag(&mut p, end, &mut len, (0x20 | 0x10) as i32) };
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataVersion(&mut p, end, unsafe { &mut (*signedData).version });
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataDigestAlgs(&mut p, end, unsafe { &mut (*signedData).digestAlgIds });
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataContentInfo(&mut p, end, unsafe { &mut (*signedData).content });
    if rc != 0 {
        return rc;
    }

    if p as usize >= end as usize {
        return crate::types::PKCS7_PARSING_ERROR as i32;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataCerts(&mut p, end, unsafe { &mut (*signedData).certs });
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataCrl(&mut p, end, unsafe { &mut (*signedData).crl });
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataSignerInfos(&mut p, end, unsafe { &mut (*signedData).signers });
    let appverify_str: &[u8] = b"appverify\0";
    let fmt_str: &[u8] = b"[%s:%d]: ParseSignedData %d\0";
    let func_str: &[u8] = b"ParseSignedData\0";
    unsafe {
        HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            (appverify_str.as_ptr)() as *const ::core::ffi::c_char,
            (fmt_str.as_ptr)() as *const ::core::ffi::c_char,
            (func_str.as_ptr)() as *const ::core::ffi::c_char,
            629 as ::core::ffi::c_int,
            rc,
        );
    }
    rc
}