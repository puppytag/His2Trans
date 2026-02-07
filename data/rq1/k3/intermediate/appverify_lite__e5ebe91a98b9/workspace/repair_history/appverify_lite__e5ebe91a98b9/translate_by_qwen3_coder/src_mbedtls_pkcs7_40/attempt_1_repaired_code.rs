fn ConstructSignerCerts(signedData: *mut crate::types::SignedData) -> i32 {
    unsafe {
        let mut signer = &mut (*signedData).signers as *mut crate::types::SignerInfo;
        while !signer.is_null() {
            let signerSerial = &(*signer).serial as *const crate::types::mbedtls_x509_buf;
            let signerIssuer = &(*signer).issuer as *const crate::types::mbedtls_x509_name;
            let mut cert = (*signedData).certs;
            let _ = crate::compat::HiLogPrint(
                crate::types::LOG_CORE as u32,
                crate::types::LOG_INFO as u32,
                0xD001100,
                b"appverify\0".as_ptr() as *const i8,
                b"[%s:%d]: To filter one signer's cert\0".as_ptr() as *const i8,
                b"ConstructSignerCerts\0".as_ptr() as *const i8,
                809,
            );
            while !cert.is_null() {
                if crate::src_mbedtls_pkcs7::SerialCmp(
                    signerSerial,
                    &(*cert).serial as *const crate::types::mbedtls_x509_buf,
                ) == 0
                    && crate::src_mbedtls_pkcs7::CompareX509NameList(
                        signerIssuer,
                        &(*cert).issuer as *const crate::types::mbedtls_x509_name,
                    ) == 0
                {
                    let _ = crate::compat::HiLogPrint(
                        crate::types::LOG_CORE as u32,
                        crate::types::LOG_INFO as u32,
                        0xD001100,
                        b"appverify\0".as_ptr() as *const i8,
                        b"[%s:%d]: Found signer's low level cert\0".as_ptr() as *const i8,
                        b"ConstructSignerCerts\0".as_ptr() as *const i8,
                        813,
                    );
                    break;
                }
                cert = (*cert).next;
            }
            if cert.is_null() {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE as u32,
                    crate::types::LOG_ERROR as u32,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const i8,
                    b"[%s:%d]: Could not found signer's lowest cert\0".as_ptr() as *const i8,
                    b"ConstructSignerCerts\0".as_ptr() as *const i8,
                    819,
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