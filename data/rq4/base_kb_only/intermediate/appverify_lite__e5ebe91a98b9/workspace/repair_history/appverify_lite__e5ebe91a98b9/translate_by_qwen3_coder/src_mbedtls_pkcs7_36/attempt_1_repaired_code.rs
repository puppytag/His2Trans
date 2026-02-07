fn FindSuperCert(cur: *mut crate::types::mbedtls_x509_crt, certsList: *mut crate::types::mbedtls_x509_crt) -> *mut crate::types::mbedtls_x509_crt {
    let mut certsList = certsList;
    
    while !certsList.is_null() {
        unsafe {
            let issuer_ptr = &(*cur).issuer as *const crate::types::mbedtls_x509_name;
            let subject_ptr = &(*certsList).subject as *const crate::types::mbedtls_x509_name;
            
            if CompareX509NameList(issuer_ptr, subject_ptr) == 0 {
                break;
            }
            certsList = (*certsList).next;
        }
    }
    certsList
}