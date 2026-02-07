fn CompareX509String(first: *const crate::types::mbedtls_x509_buf, second: *const crate::types::mbedtls_x509_buf) -> i32 {
    unsafe {
        let first_tag = (*first).tag;
        let second_tag = (*second).tag;
        
        if crate::src_mbedtls_pkcs7::IsLegitString(first_tag) && crate::src_mbedtls_pkcs7::IsLegitString(second_tag) {
            let first_len = (*first).len as i32;
            let first_p = (*first).p;
            let second_p = (*second).p;
            
            for i in 0..first_len {
                let fc = *first_p.offset(i as isize) as i32;
                let sc = *second_p.offset(i as isize) as i32;
                
                let is_lower = ((fc as u32).wrapping_sub('a' as u32)) < 26;
                let is_upper = ((fc as u32).wrapping_sub('A' as u32)) < 26;
                
                if fc == sc ||
                   (is_lower && (fc - 32 == sc)) ||
                   (is_upper && (fc + 32 == sc)) {
                    continue;
                }
                return -1;
            }
            return 0;
        }
        -1
    }
}