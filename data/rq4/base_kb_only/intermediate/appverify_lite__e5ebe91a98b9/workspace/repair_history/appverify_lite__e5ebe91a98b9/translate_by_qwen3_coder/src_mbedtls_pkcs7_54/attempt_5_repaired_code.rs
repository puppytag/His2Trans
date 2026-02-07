pub extern "C" fn PKCS7_VerifyCertsChain(pkcs7: *const crate::types::Pkcs7) -> i32 {
    if pkcs7.is_null() {
        return crate::types::PKCS7_INVALID_PARAM as i32;
    }
    
    let mut cnt: i32 = 0;
    let mut signer: *const crate::types::SignerInfo = unsafe { &(*pkcs7).signedData.signers };
    
    while !signer.is_null() {
        let clicert: *mut crate::types::mbedtls_x509_crt = unsafe { (*signer).certPath.crt };
        
        if clicert.is_null() {
            return crate::types::PKCS7_HAS_NO_SIGNER_CRT as i32;
        }
        
        let mut rc: i32;
        cnt += 1;
        let _ = cnt;
        
        unsafe {
            if g_debugModeEnabled {
                rc = VerifyClicert(
                    clicert,
                    std::ptr::addr_of_mut!(g_debugModeRootCert),
                    pkcs7,
                );
                if rc == crate::types::PKCS7_SUCC as i32 {
                    signer = (*signer).next as *const _;
                    continue;
                }
                if rc == crate::types::PKCS7_IS_REVOKED as i32 {
                    return crate::types::PKCS7_IS_REVOKED as i32;
                }
            }
            
            rc = VerifyClicert(clicert, (*signer).rootCert, pkcs7);
            if rc == crate::types::PKCS7_SUCC as i32 {
                signer = (*signer).next as *const _;
                continue;
            }
            if rc == crate::types::PKCS7_IS_REVOKED as i32 {
                return crate::types::PKCS7_IS_REVOKED as i32;
            }
            
            rc = VerifyClicert(
                clicert,
                std::ptr::addr_of_mut!(g_ohosRootCert),
                pkcs7,
            );
            if rc == crate::types::PKCS7_SUCC as i32 {
                signer = (*signer).next as *const _;
                continue;
            }
        }
        
        return rc;
    }
    
    crate::types::PKCS7_SUCC as i32
}