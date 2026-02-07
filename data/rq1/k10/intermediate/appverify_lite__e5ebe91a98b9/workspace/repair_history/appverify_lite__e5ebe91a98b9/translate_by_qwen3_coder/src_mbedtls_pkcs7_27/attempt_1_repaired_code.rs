fn ParseSignedData(buf: *mut u8, bufLen: usize, signedData: *mut crate::types::SignedData) -> i32 {
    let mut p = buf;
    let end = unsafe { buf.add(bufLen) };
    let mut len: crate::types::size_t = 0;
    let mut rc: i32;

    rc = unsafe { crate::compat::mbedtls_asn1_get_tag(&mut p as *mut *mut u8, end as *const u8, &mut len, 0x20 | 0x10) };
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataVersion(
        &mut p as *mut *mut u8,
        end as *const u8,
        unsafe { &mut (*signedData).version } as *mut i32,
    );
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataDigestAlgs(
        &mut p as *mut *mut ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        unsafe { &mut (*signedData).digestAlgIds } as *mut crate::types::DigestAlgId,
    );
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataContentInfo(
        &mut p as *mut *mut ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        unsafe { &mut (*signedData).content } as *mut crate::types::Content,
    );
    if rc != 0 {
        return rc;
    }

    if p >= end {
        return crate::types::PKCS7_PARSING_ERROR as i32;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataCerts(
        &mut p as *mut *mut ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        unsafe { &mut (*signedData).certs } as *mut *mut crate::types::mbedtls_x509_crt,
    );
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataCrl(
        &mut p as *mut *mut ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        unsafe { &mut (*signedData).crl } as *mut crate::types::mbedtls_x509_crl,
    );
    if rc != 0 {
        return rc;
    }

    rc = crate::src_mbedtls_pkcs7::ParseSignedDataSignerInfos(
        &mut p as *mut *mut ::core::ffi::c_uchar,
        end as *const ::core::ffi::c_uchar,
        unsafe { &mut (*signedData).signers } as *mut crate::types::SignerInfo,
    );
    let _ = unsafe { crate::compat::HiLogPrint(
        crate::types::LOG_CORE,
        crate::types::LOG_INFO,
        0xD001100,
        "appverify\0".as_ptr() as *const ::core::ffi::c_char,
        "[%s:%d]: ParseSignedData %d\0".as_ptr() as *const ::core::ffi::c_char,
        "ParseSignedData\0".as_ptr() as *const ::core::ffi::c_char,
        629,
        rc,
    ) };
    return rc;
}