fn VerifyBinSign(signInfo: *mut crate::types::SignatureInfo, fp: i32, signCert: *mut *mut crate::types::CertInfo, certType: *mut i32) -> i32 {
    let mut blockLen: i32 = 0;
    let mut blockHead = crate::types::BlockHead {
        type_: 0,
        length: 0,
        offset: 0,
    };
    let mut fileRead: *mut crate::types::FileRead = std::ptr::null_mut();
    let mut ret: i32 = 0;

    let signBuf = crate::src_app_verify::GetSignBlockByType(signInfo as *const _, fp, crate::types::SIGNATURE_BLOCK_TYPE as i32, &mut blockLen, &mut blockHead);
    if signBuf.is_null() {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: signBuf is null\0".as_ptr() as *const _, b"VerifyBinSign\0".as_ptr() as *const _, 1100);
        return crate::types::V_ERR_GET_SIGN_BLOCK as i32;
    }

    let pkcs7 = crate::src_app_verify::GetBinSignPkcs(signBuf, blockLen);
    if pkcs7.is_null() {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: GetBinSignPkcs failed\0".as_ptr() as *const _, b"VerifyBinSign\0".as_ptr() as *const _, 1104);
        unsafe { libc::free(signBuf as *mut _) };
        return crate::types::V_ERR_PARSE_PKC7_DATA as i32;
    }

    let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: pkcs7 parse message success\0".as_ptr() as *const _, b"VerifyBinSign\0".as_ptr() as *const _, 1109);

    fileRead = crate::src_app_verify::GetFileRead(fp, 0, blockHead.offset as i32);
    if fileRead.is_null() {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: malloc error\0".as_ptr() as *const _, b"VerifyBinSign\0".as_ptr() as *const _, 1114);
        ret = crate::types::V_ERR_MALLOC as i32;
        goto cleanup;
    }
    ret = crate::src_app_verify::GetAppSingerCertType(pkcs7, certType);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: cert source invalid: %d\0".as_ptr() as *const _, b"VerifyBinSign\0".as_ptr() as *const _, 1120, ret);
        ret = crate::types::V_ERR_GET_CERT_TYPE as i32;
        goto cleanup;
    }
    let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: get cert Type : %d\0".as_ptr() as *const _, b"VerifyBinSign\0".as_ptr() as *const _, 1124, unsafe { *certType });
    unsafe { (*signInfo).certType = *certType };
    ret = crate::src_app_verify::VerifyAppSignPkcsData(fileRead as *const _, signInfo as *const _, pkcs7 as *const _);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: intergrity failed\0".as_ptr() as *const _, b"VerifyBinSign\0".as_ptr() as *const _, 1128);
        ret = crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
        goto cleanup;
    }
    let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: pkcs7 verify signer signature success\0".as_ptr() as *const _, b"VerifyBinSign\0".as_ptr() as *const _, 1132);

    let crt_ptr = unsafe { (*pkcs7).signedData.signers.certPath.crt };
    ret = crate::src_app_verify::GetCertInfo(crt_ptr as *const _, signCert);
    if ret != crate::types::V_OK as i32 {
        let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_ERROR, 0xD001100, b"appverify\0".as_ptr() as *const _, b"[%s:%d]: get bin cert info  error: %d\0".as_ptr() as *const _, b"VerifyBinSign\0".as_ptr() as *const _, 1136, ret);
        ret = crate::types::V_ERR_GET_CERT_INFO as i32;
        goto cleanup;
    }

cleanup:
    if !signBuf.is_null() {
        unsafe { libc::free(signBuf as *mut _) };
    }
    crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
    if !pkcs7.is_null() {
        unsafe { libc::free(pkcs7 as *mut _) };
    }
    if !fileRead.is_null() {
        unsafe { libc::free(fileRead as *mut _) };
    }
    ret
}