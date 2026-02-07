fn VerifyCrl(crt: *const crate::types::mbedtls_x509_crt, crl: *const crate::types::mbedtls_x509_crl) -> i32 {
    let mut crl_list = crl;
    while !crl_list.is_null() {
        unsafe {
            if (*crl_list).version == 0 ||
                crate::src_mbedtls_pkcs7::CompareX509NameList(&(*crl_list).issuer, &(*crt).issuer) != 0 {
                crl_list = (*crl_list).next;
                continue;
            }
        }
        unsafe {
            let _ = crate::compat::HiLogPrint(crate::types::LOG_CORE, crate::types::LOG_INFO, 0xD001100, "appverify\0".as_ptr() as *const i8, "[%s:%d]: find crl\0".as_ptr() as *const i8, "VerifyCrl\0".as_ptr() as *const i8, 1012);
        }
        if crate::src_mbedtls_pkcs7::IsRevoked(crt, crl_list) != 0 {
            return crate::types::PKCS7_IS_REVOKED as i32;
        }
        unsafe {
            crl_list = (*crl_list).next;
        }
    }
    crate::types::PKCS7_SUCC as i32
}