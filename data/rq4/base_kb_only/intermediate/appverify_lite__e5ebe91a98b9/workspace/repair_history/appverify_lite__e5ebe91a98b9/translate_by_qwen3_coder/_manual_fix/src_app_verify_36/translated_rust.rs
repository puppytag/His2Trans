fn VerifyAppSignPkcsData(fileRead: *const crate::types::FileRead, signInfo: *const crate::types::SignatureInfo, pkcs7Handle: *const crate::types::Pkcs7) -> i32 {
    let mut ret = crate::src_mbedtls_pkcs7::PKCS7_VerifyCertsChain(pkcs7Handle);
    if ret != crate::types::V_OK as i32 {
        return crate::types::V_ERR_VERIFY_CERT_CHAIN as i32;
    }

    ret = crate::src_app_verify::VerifyRawHash(signInfo, fileRead, pkcs7Handle);
    if ret != crate::types::V_OK as i32 {
        return ret;
    }

    ret = crate::src_mbedtls_pkcs7::PKCS7_VerifySignerSignature(pkcs7Handle, Some(crate::src_app_verify::CalcDigest as extern "C" fn(*const crate::types::Pkcs7, *const crate::types::SignerInfo, crate::types::mbedtls_md_type_t, *mut u8, *mut crate::types::size_t) -> i32));
    if ret != crate::types::V_OK as i32 {
        return crate::types::V_ERR_VERIFY_SIGNATURE as i32;
    }

    crate::types::V_OK as i32
}