fn CalcCmpContHash(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut ::core::ffi::c_uchar, hashLen: *mut ::core::ffi::c_uint) -> i32 {
    let mut rc: i32;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;

    rc = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7, &mut input, &mut inputLen);
    if rc != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as i32, crate::types::LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: rc not ok\0".as_ptr() as *const _, b"CalcCmpContHash\0".as_ptr() as *const _, 260);
        return rc;
    }

    unsafe {
        rc = crate::compat::mbedtls_md(crate::compat::mbedtls_md_info_from_type(algType), input, inputLen as usize, hash);
    }
    if rc != 0 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as i32, crate::types::LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Error: calc digest failed\0".as_ptr() as *const _, b"CalcCmpContHash\0".as_ptr() as *const _, 264);
        return rc;
    }
    unsafe {
        *hashLen = crate::compat::mbedtls_md_get_size(crate::compat::mbedtls_md_info_from_type(algType)) as ::core::ffi::c_uint;
    }

    let mut digInAttr: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut digInAttrLen: crate::types::size_t = 0;
    rc = crate::src_mbedtls_pkcs7::PKCS7_GetDigestInSignerAuthAttr(signer, &mut digInAttr, &mut digInAttrLen);
    if rc != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as i32, crate::types::LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: PKCS7_GetDigestInSignerAuthAttr error: %d\0".as_ptr() as *const _, b"CalcCmpContHash\0".as_ptr() as *const _, 274, rc);
        return rc;
    }
    unsafe {
        if digInAttrLen != *hashLen as crate::types::size_t {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as i32, crate::types::LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Error: content hash len is not equal with attr's hash len\0".as_ptr() as *const _, b"CalcCmpContHash\0".as_ptr() as *const _, 278);
            return crate::types::V_ERR as i32;
        }
        if crate::compat::memcmp(hash as *const _, digInAttr as *const _, digInAttrLen as usize) != 0 {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE as i32, crate::types::LOG_ERROR as i32, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: Error: content hash not equal with attr hash\0".as_ptr() as *const _, b"CalcCmpContHash\0".as_ptr() as *const _, 282);
            return crate::types::V_ERR as i32;
        }
    }
    crate::types::V_OK as i32
}