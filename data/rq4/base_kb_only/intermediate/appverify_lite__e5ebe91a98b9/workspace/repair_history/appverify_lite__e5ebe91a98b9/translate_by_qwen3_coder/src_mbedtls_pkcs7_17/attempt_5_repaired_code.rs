fn CompareX509NameList(first: *const crate::types::mbedtls_x509_name, second: *const crate::types::mbedtls_x509_name) -> i32 {
    if first.is_null() || second.is_null() {
        return -1;
    }
    
    let first_deps = GetDeps(first);
    let second_deps = GetDeps(second);
    
    if first_deps != second_deps {
        return -1;
    }
    
    let mut first_ptr = first;
    let mut second_ptr = second;
    
    let mut i: i32 = 0;
    while i < first_deps {
        unsafe {
            if (*first_ptr).oid.tag != (*second_ptr).oid.tag ||
               (*first_ptr).oid.len != (*second_ptr).oid.len ||
               libc::memcmp(
                   (*first_ptr).oid.p as *const core::ffi::c_void,
                   (*second_ptr).oid.p as *const core::ffi::c_void,
                   (*second_ptr).oid.len as usize
               ) != 0 ||
               (*first_ptr).private_next_merged != (*second_ptr).private_next_merged ||
               (*first_ptr).val.len != (*second_ptr).val.len {
                return -1;
            }
            
            let first_val_ptr = &(*first_ptr).val as *const crate::types::mbedtls_asn1_buf;
            let second_val_ptr = &(*second_ptr).val as *const crate::types::mbedtls_asn1_buf;
            
            if CompareX509String(first_val_ptr, second_val_ptr) != 0 {
                return -1;
            }
            
            first_ptr = (*first_ptr).next as *const crate::types::mbedtls_x509_name;
            second_ptr = (*second_ptr).next as *const crate::types::mbedtls_x509_name;
        }
        i += 1;
    }
    
    0
}