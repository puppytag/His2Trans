extern "C" fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    let mut ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7Handle);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: Verify certs failed, ret: %d\0".as_ptr() as *const i8,
            "VerifyAppSignPkcsData\0".as_ptr() as *const i8,
            1041,
            ret,
        );
        return crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
    }
    let _ = crate::compat::HiLogPrint(
        crate::types::LOG_CORE as u32,
        crate::types::LOG_INFO as u32,
        0xD001100,
        "appverify\0".as_ptr() as *const i8,
        "[%s:%d]: Verify certs success\0".as_ptr() as *const i8,
        "VerifyAppSignPkcsData\0".as_ptr() as *const i8,
        1044,
    );
    ret = crate::src_app_verify::VerifyRawHash(signInfo, fileRead, pkcs7Handle);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: VerifyRawHash failed : %d\0".as_ptr() as *const i8,
            "VerifyAppSignPkcsData\0".as_ptr() as *const i8,
            1048,
            ret,
        );
        return ret;
    }
    let _ = crate::compat::HiLogPrint(
        crate::types::LOG_CORE as u32,
        crate::types::LOG_INFO as u32,
        0xD001100,
        "appverify\0".as_ptr() as *const i8,
        "[%s:%d]: VerifyRawHash success\0".as_ptr() as *const i8,
        "VerifyAppSignPkcsData\0".as_ptr() as *const i8,
        1051,
    );
    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7Handle, Some(crate::src_app_verify::CalcDigest));
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE as u32,
            crate::types::LOG_ERROR as u32,
            0xD001100,
            "appverify\0".as_ptr() as *const i8,
            "[%s:%d]: pkcs7 verify signer signature failed : %d\0".as_ptr() as *const i8,
            "VerifyAppSignPkcsData\0".as_ptr() as *const i8,
            1055,
            ret,
        );
        return crate::types::V_ERR_VERIFY_SIGNATURE as i32;
    }
    crate::types::V_OK as i32
}