extern "C" fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    let mut ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7Handle);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: Verify certs failed, ret: %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyAppSignPkcsData\0".as_ptr() as *const ::core::ffi::c_char,
                1041i32,
                ret,
            );
        }
        return crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
    }
    unsafe {
        let _ = HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: Verify certs success\0".as_ptr() as *const ::core::ffi::c_char,
            b"VerifyAppSignPkcsData\0".as_ptr() as *const ::core::ffi::c_char,
            1044i32,
        );
    }

    ret = crate::src_app_verify::VerifyRawHash(signInfo, fileRead, pkcs7Handle);
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: VerifyRawHash failed : %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyAppSignPkcsData\0".as_ptr() as *const ::core::ffi::c_char,
                1048i32,
                ret,
            );
        }
        return ret;
    }
    unsafe {
        let _ = HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_INFO,
            0xD001100,
            b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
            b"[%s:%d]: VerifyRawHash success\0".as_ptr() as *const ::core::ffi::c_char,
            b"VerifyAppSignPkcsData\0".as_ptr() as *const ::core::ffi::c_char,
            1051i32,
        );
    }

    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7Handle, Some(CalcDigest));
    if ret != crate::types::V_OK as i32 {
        unsafe {
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_ERROR,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: pkcs7 verify signer signature failed : %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"VerifyAppSignPkcsData\0".as_ptr() as *const ::core::ffi::c_char,
                1055i32,
                ret,
            );
        }
        return crate::types::V_ERR_VERIFY_SIGNATURE as i32;
    }

    crate::types::V_OK as i32
}