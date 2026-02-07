fn FindSuperCert(cur: *mut crate::types::mbedtls_x509_crt, certsList: *mut crate::types::mbedtls_x509_crt) -> *mut crate::types::mbedtls_x509_crt {
    let mut certsList = certsList;
    
    while certsList != std::ptr::null_mut() {
        unsafe {
            let issuer_ptr = &(*cur).issuer as *const crate::types::mbedtls_x509_name;
            let subject_ptr = &(*certsList).subject as *const crate::types::mbedtls_x509_name;
            
            if crate::src_mbedtls_pkcs7::CompareX509NameList(issuer_ptr, subject_ptr) == 0 {
                break;
            }
            certsList = (*certsList).next;
        }
    }
    certsList
}