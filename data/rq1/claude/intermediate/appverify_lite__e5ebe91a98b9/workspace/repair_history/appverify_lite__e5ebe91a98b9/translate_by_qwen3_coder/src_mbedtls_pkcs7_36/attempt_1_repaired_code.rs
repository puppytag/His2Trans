fn FindSuperCert(cur: *mut crate::types::mbedtls_x509_crt, certsList: *mut crate::types::mbedtls_x509_crt) -> *mut crate::types::mbedtls_x509_crt {
    let mut certs_list = certsList;
    while !certs_list.is_null() {
        unsafe {
            let issuer_ptr = &(*cur).issuer as *const crate::types::mbedtls_x509_name;
            let subject_ptr = &(*certs_list).subject as *const crate::types::mbedtls_x509_name;
            if crate::src_mbedtls_pkcs7::CompareX509NameList(issuer_ptr, subject_ptr) == 0 {
                break;
            }
            certs_list = (*certs_list).next;
        }
    }
    certs_list
}