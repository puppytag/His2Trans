fn IsIncludeRoot(signer: *const crate::types::SignerInfo) -> bool {
    if signer.is_null() {
        return false;
    }
    
    unsafe {
        let mut pre: *mut crate::types::mbedtls_x509_crt = (*signer).certPath.crt;
        let mut cur: *mut crate::types::mbedtls_x509_crt = pre;
        let mut i: i32 = 0;
        
        while i < (*signer).certPath.depth && !cur.is_null() {
            pre = cur;
            cur = (*cur).next;
            i += 1;
        }
        
        if pre.is_null() {
            return false;
        }
        
        if CompareX509NameList(
            &(*pre).issuer as *const crate::types::mbedtls_x509_name,
            &(*pre).subject as *const crate::types::mbedtls_x509_name
        ) == 0 {
            return true;
        }
        
        false
    }
}