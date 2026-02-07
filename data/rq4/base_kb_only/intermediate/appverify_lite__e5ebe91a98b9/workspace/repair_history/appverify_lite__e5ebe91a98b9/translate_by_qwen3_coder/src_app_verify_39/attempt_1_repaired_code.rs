fn VerifyBinSign(signInfo: *mut crate::types::SignatureInfo, fp: i32, signCert: *mut *mut crate::types::CertInfo, certType: *mut i32) -> i32 {
    let mut blockLen: i32 = 0;
    let mut blockHead: crate::types::BlockHead = crate::types::BlockHead {
        type_: 0,
        length: 0,
        offset: 0,
    };
    let mut fileRead: *mut crate::types::FileRead = std::ptr::null_mut();
    let mut ret: i32;

    let signBuf = GetSignBlockByType(
        signInfo as *const crate::types::SignatureInfo,
        fp,
        crate::types::SIGNATURE_BLOCK_TYPE as i32,
        &mut blockLen,
        &mut blockHead,
    );

    if signBuf.is_null() {
        return crate::types::V_ERR_GET_SIGN_BLOCK as i32;
    }

    let pkcs7 = GetBinSignPkcs(signBuf as *const i8, blockLen);
    if pkcs7.is_null() {
        if !signBuf.is_null() {
            unsafe { libc::free(signBuf as *mut core::ffi::c_void) };
        }
        return crate::types::V_ERR_PARSE_PKC7_DATA as i32;
    }

    fileRead = GetFileRead(fp, 0, blockHead.offset as i32);
    if fileRead.is_null() {
        ret = crate::types::V_ERR_MALLOC as i32;
    } else {
        ret = GetAppSingerCertType(pkcs7, certType);
        if ret != crate::types::V_OK as i32 {
            ret = crate::types::V_ERR_GET_CERT_TYPE as i32;
        } else {
            unsafe {
                (*signInfo).certType = *certType;
            }
            ret = VerifyAppSignPkcsData(
                fileRead as *const crate::types::FileRead,
                signInfo as *const crate::types::SignatureInfo,
                pkcs7 as *const crate::types::Pkcs7,
            );
            if ret != crate::types::V_OK as i32 {
                ret = crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
            } else {
                let crt = unsafe { (*pkcs7).signedData.signers.certPath.crt };
                ret = GetCertInfo(
                    crt as *const crate::types::mbedtls_x509_crt,
                    signCert,
                );
                if ret != crate::types::V_OK as i32 {
                    ret = crate::types::V_ERR_GET_CERT_INFO as i32;
                }
            }
        }
    }

    if !signBuf.is_null() {
        unsafe { libc::free(signBuf as *mut core::ffi::c_void) };
    }

    PKCS7_FreeRes(pkcs7);
    if !pkcs7.is_null() {
        unsafe { libc::free(pkcs7 as *mut core::ffi::c_void) };
    }
    if !fileRead.is_null() {
        unsafe { libc::free(fileRead as *mut core::ffi::c_void) };
    }

    ret
}