fn BuildSignerCertPath(signer: *mut crate::types::SignerInfo, lowerCrt: *mut crate::types::mbedtls_x509_crt, signeData: *mut crate::types::SignedData) -> i32 {
    let mut scanCnt: i32 = 0;
    let mut rc: i32 = crate::types::PKCS7_SUCC as i32;
    
    unsafe {
        if crate::globals::g_rootCertLoaded == false {
            return crate::types::PKCS7_ROOT_CA_NOT_VALID as i32;
        }
        (*signer).rootCert = std::ptr::addr_of_mut!(crate::globals::g_rootCaG2Cert);
        
        let mut cur: *mut crate::types::mbedtls_x509_crt = lowerCrt;
        let mut next: *mut crate::types::mbedtls_x509_crt = std::ptr::null_mut();
        let certsCnt: i32 = crate::src_mbedtls_pkcs7::GetCertsNumOfSignedData((*signeData).certs as *const crate::types::mbedtls_x509_crt);
        crate::src_mbedtls_pkcs7::DelCertOfSignedData(signeData, cur);
        crate::src_mbedtls_pkcs7::AddCertToSignerCertPath(signer, cur);
        
        loop {
            next = crate::src_mbedtls_pkcs7::FindSuperCert(cur, (*signeData).certs);
            if next.is_null() {
                break;
            } else {
                crate::src_mbedtls_pkcs7::DelCertOfSignedData(signeData, next);
                crate::src_mbedtls_pkcs7::AddCertToSignerCertPath(signer, next);
            }
            scanCnt += 1;
            if scanCnt > certsCnt {
                rc = crate::types::PKCS7_BUILD_CERT_PATH_FAIL as i32;
                break;
            }
            cur = next;
        }
    }
    rc
}