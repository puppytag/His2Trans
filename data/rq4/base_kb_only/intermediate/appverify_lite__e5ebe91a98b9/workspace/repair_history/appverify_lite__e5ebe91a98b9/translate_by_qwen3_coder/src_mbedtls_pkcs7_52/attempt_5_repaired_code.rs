fn VerifyCrl(crt: *const crate::types::mbedtls_x509_crt, crl: *const crate::types::mbedtls_x509_crl) -> i32 {
    let mut crlList = crl;
    while !crlList.is_null() {
        unsafe {
            if (*crlList).version == 0 ||
               crate::src_mbedtls_pkcs7::CompareX509NameList(
                   &(*crlList).issuer as *const crate::types::mbedtls_x509_name,
                   &(*crt).issuer as *const crate::types::mbedtls_x509_name
               ) != 0 {
                crlList = (*crlList).next;
                continue;
            }
            if crate::src_mbedtls_pkcs7::IsRevoked(crt, crlList) != 0 {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
            crlList = (*crlList).next;
        }
    }
    crate::types::PKCS7_SUCC as i32
}