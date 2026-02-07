fn VerifyBinSign(signInfo: *mut crate::types::SignatureInfo, fp: i32, signCert: *mut *mut crate::types::CertInfo, certType: *mut i32) -> i32 {
    let mut blockLen: i32 = 0;
    let mut blockHead = crate::types::BlockHead {
        type_: 0,
        length: 0,
        offset: 0,
    };
    let mut fileRead: *mut crate::types::FileRead = std::ptr::null_mut();
    let mut ret: i32;

    let signBuf = crate::src_app_verify::GetSignBlockByType(signInfo as *const _, fp, crate::types::SIGNATURE_BLOCK_TYPE as i32, &mut blockLen, &mut blockHead);
    if signBuf.is_null() {
        return crate::types::V_ERR_GET_SIGN_BLOCK as i32;
    }

    let pkcs7 = crate::src_app_verify::GetBinSignPkcs(signBuf as *const std::ffi::c_char, blockLen);
    if pkcs7.is_null() {
        unsafe { libc::free(signBuf as *mut std::ffi::c_void) };
        return crate::types::V_ERR_PARSE_PKC7_DATA as i32;
    }

    fileRead = crate::src_app_verify::GetFileRead(fp, 0, blockHead.offset as i32);
    if fileRead.is_null() {
        ret = crate::types::V_ERR_MALLOC as i32;
        unsafe { libc::free(signBuf as *mut std::ffi::c_void) };
        crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
        unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
        return ret;
    }

    ret = crate::src_app_verify::GetAppSingerCertType(pkcs7, certType);
    if ret != crate::types::V_OK as i32 {
        ret = crate::types::V_ERR_GET_CERT_TYPE as i32;
        goto_exit;
    }

    unsafe { (*signInfo).certType = *certType; }

    ret = crate::src_app_verify::VerifyAppSignPkcsData(fileRead as *const _, signInfo as *const _, pkcs7 as *const _);
    if ret != crate::types::V_OK as i32 {
        ret = crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
        goto_exit;
    }

    let crt_ptr = unsafe { (*pkcs7).signedData.signers.certPath.crt };
    ret = crate::src_app_verify::GetCertInfo(crt_ptr as *const _, signCert);
    if ret != crate::types::V_OK as i32 {
        ret = crate::types::V_ERR_GET_CERT_INFO as i32;
        goto_exit;
    }

    ret = crate::types::V_OK as i32;

    goto_exit;

    macro_rules! goto_exit {
        () => {
            unsafe { libc::free(signBuf as *mut std::ffi::c_void) };
            crate::src_mbedtls_pkcs7::PKCS7_FreeRes(pkcs7);
            unsafe { libc::free(pkcs7 as *mut std::ffi::c_void) };
            unsafe { libc::free(fileRead as *mut std::ffi::c_void) };
            return ret;
        };
    }
}