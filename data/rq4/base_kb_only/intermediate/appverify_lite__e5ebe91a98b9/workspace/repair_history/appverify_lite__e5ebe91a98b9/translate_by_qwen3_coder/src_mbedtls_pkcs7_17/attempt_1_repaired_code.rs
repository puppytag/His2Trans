fn CompareX509NameList(first: *const crate::types::mbedtls_x509_name, second: *const crate::types::mbedtls_x509_name) -> i32 {
    if first.is_null() || second.is_null() {
        return -1;
    }
    
    let firstDeps = GetDeps(first);
    let secondDeps = GetDeps(second);
    
    if firstDeps != secondDeps {
        return -1;
    }
    
    let mut first = first;
    let mut second = second;
    
    for _i in 0..firstDeps {
        unsafe {
            if (*first).oid.tag != (*second).oid.tag ||
               (*first).oid.len != (*second).oid.len ||
               libc::memcmp(
                   (*first).oid.p as *const core::ffi::c_void,
                   (*second).oid.p as *const core::ffi::c_void,
                   (*second).oid.len as usize
               ) != 0 ||
               (*first).private_next_merged != (*second).private_next_merged ||
               (*first).val.len != (*second).val.len {
                return -1;
            }
            
            if CompareX509String(
                &(*first).val as *const crate::types::mbedtls_x509_buf,
                &(*second).val as *const crate::types::mbedtls_x509_buf
            ) != 0 {
                return -1;
            }
            
            first = (*first).next as *const crate::types::mbedtls_x509_name;
            second = (*second).next as *const crate::types::mbedtls_x509_name;
        }
    }
    
    0
}