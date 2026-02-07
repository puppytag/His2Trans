fn CalcDigest(pkcs7: *const crate::types::Pkcs7, signer: *const crate::types::SignerInfo, algType: crate::types::mbedtls_md_type_t, hash: *mut ::core::ffi::c_uchar, hashLen: *mut ::core::ffi::c_uint) -> i32 {
    let mut rc: i32;
    let mut input: *mut ::core::ffi::c_uchar = std::ptr::null_mut();
    let mut inputLen: crate::types::size_t = 0;

    rc = crate::src_app_verify::CalcCmpContHash(pkcs7, signer, algType, hash, hashLen);
    if rc != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: content hash not equal with attr hash\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcDigest\0".as_ptr() as *const ::core::ffi::c_char,
                296 as ::core::ffi::c_int,
            );
        }
        return rc;
    }
    unsafe {
        crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: signer context hash equal with attr hash\0".as_ptr() as *const ::core::ffi::c_char,
            b"CalcDigest\0".as_ptr() as *const ::core::ffi::c_char,
            299 as ::core::ffi::c_int,
        );
    }

    rc = unsafe { crate::src_mbedtls_pkcs7::PKCS7_GetSignerAuthAttr(signer, &mut input, &mut inputLen) };
    if rc != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: PKCS7_GetSignerAuthAttr failed ret: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcDigest\0".as_ptr() as *const ::core::ffi::c_char,
                304 as ::core::ffi::c_int,
                rc,
            );
        }
        return rc;
    }

    rc = unsafe {
        crate::compat::mbedtls_md(
            crate::compat::mbedtls_md_info_from_type(algType),
            input,
            inputLen as u32,
            hash,
        )
    };
    if rc != crate::types::V_OK as i32 {
        unsafe {
            crate::compat::HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Error: calc digest failed ret: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"CalcDigest\0".as_ptr() as *const ::core::ffi::c_char,
                309 as ::core::ffi::c_int,
                rc,
            );
        }
        return rc;
    }

    unsafe {
        *hashLen = crate::compat::mbedtls_md_get_size(crate::compat::mbedtls_md_info_from_type(algType)) as ::core::ffi::c_uint;
    }

    crate::types::V_OK as i32
}