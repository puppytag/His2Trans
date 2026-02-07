fn ConstructSignerCerts(signedData: *mut crate::types::SignedData) -> i32 {
    unsafe {
        let mut signer: *mut crate::types::SignerInfo = &mut (*signedData).signers;
        while !signer.is_null() {
            let signerSerial: *const crate::types::mbedtls_x509_buf = &(*signer).serial;
            let signerIssuer: *const crate::types::mbedtls_x509_name = &(*signer).issuer;
            let mut cert: *mut crate::types::mbedtls_x509_crt = (*signedData).certs;
            
            let _ = HiLogPrint(
                crate::types::LOG_CORE,
                crate::types::LOG_INFO,
                0xD001100,
                b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                b"[%s:%d]: To filter one signer's cert\0".as_ptr() as *const ::core::ffi::c_char,
                b"ConstructSignerCerts\0".as_ptr() as *const ::core::ffi::c_char,
                809 as ::core::ffi::c_int,
            );
            
            while !cert.is_null() {
                if crate::src_mbedtls_pkcs7::SerialCmp(signerSerial, &(*cert).serial) == 0
                    && crate::src_mbedtls_pkcs7::CompareX509NameList(signerIssuer, &(*cert).issuer) == 0
                {
                    let _ = HiLogPrint(
                        crate::types::LOG_CORE,
                        crate::types::LOG_INFO,
                        0xD001100,
                        b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                        b"[%s:%d]: Found signer's low level cert\0".as_ptr() as *const ::core::ffi::c_char,
                        b"ConstructSignerCerts\0".as_ptr() as *const ::core::ffi::c_char,
                        813 as ::core::ffi::c_int,
                    );
                    break;
                }
                cert = (*cert).next;
            }
            
            if cert.is_null() {
                let _ = HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const ::core::ffi::c_char,
                    b"[%s:%d]: Could not found signer's lowest cert\0".as_ptr() as *const ::core::ffi::c_char,
                    b"ConstructSignerCerts\0".as_ptr() as *const ::core::ffi::c_char,
                    819 as ::core::ffi::c_int,
                );
                return crate::types::PKCS7_INVALID_VALUE as i32;
            }
            
            let rc = crate::src_mbedtls_pkcs7::BuildSignerCertPath(signer, cert, signedData);
            if rc != 0 {
                return rc;
            }
            signer = (*signer).next;
        }
        0
    }
}