fn FindSuperCert(cur: *mut crate::types::mbedtls_x509_crt, certsList: *mut crate::types::mbedtls_x509_crt) -> *mut crate::types::mbedtls_x509_crt {
    let mut certs_list = certsList;
    
    while !certs_list.is_null() {
        unsafe {
            let issuer_ptr: *const crate::types::mbedtls_x509_name = &(*cur).issuer;
            let subject_ptr: *const crate::types::mbedtls_x509_name = &(*certs_list).subject;
            
            if crate::src_mbedtls_pkcs7::CompareX509NameList(issuer_ptr, subject_ptr) == 0 {
                break;
            }
            certs_list = (*certs_list).next;
        }
    }
    certs_list
}