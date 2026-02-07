fn CalcCmpContHash(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut u8, hashLen: *mut crate::types::size_t) -> i32 {
    let mut rc: i32;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;

    rc = crate::src_mbedtls_pkcs7::PKCS7_GetContentData(pkcs7, &mut input, &mut inputLen);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: rc not ok\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcCmpContHash\0".as_ptr() as *const ::core::ffi::c_char,
                260i32,
            );
        }
        return rc;
    }

    let md_info = unsafe { mbedtls_md_info_from_type(algType) };
    rc = unsafe { mbedtls_md(md_info, input, inputLen, hash) };
    if rc != 0 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: calc digest failed\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcCmpContHash\0".as_ptr() as *const ::core::ffi::c_char,
                264i32,
            );
        }
        return rc;
    }
    unsafe {
        *hashLen = mbedtls_md_get_size(mbedtls_md_info_from_type(algType)) as crate::types::size_t;
    }

    let mut digInAttr: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut digInAttrLen: crate::types::size_t = 0;
    rc = crate::src_mbedtls_pkcs7::PKCS7_GetDigestInSignerAuthAttr(signer, &mut digInAttr, &mut digInAttrLen);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: PKCS7_GetDigestInSignerAuthAttr error: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcCmpContHash\0".as_ptr() as *const ::core::ffi::c_char,
                274i32,
                rc,
            );
        }
        return rc;
    }
    if digInAttrLen != unsafe { *hashLen } {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: content hash len is not equal with attr's hash len\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcCmpContHash\0".as_ptr() as *const ::core::ffi::c_char,
                278i32,
            );
        }
        return crate::types::V_ERR as i32;
    }
    if unsafe { libc::memcmp(hash as *const ::core::ffi::c_void, digInAttr as *const ::core::ffi::c_void, digInAttrLen as usize) } != 0 {
        unsafe {
            HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: content hash not equal with attr hash\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcCmpContHash\0".as_ptr() as *const ::core::ffi::c_char,
                282i32,
            );
        }
        return crate::types::V_ERR as i32;
    }
    crate::types::V_OK as i32
}