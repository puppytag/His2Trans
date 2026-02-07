fn CompareX509NameList(first: *const crate::types::mbedtls_x509_name, second: *const crate::types::mbedtls_x509_name) -> i32 {
    if first.is_null() || second.is_null() {
        return -1;
    }
    let first_deps = crate::src_mbedtls_pkcs7::GetDeps(first);
    let second_deps = crate::src_mbedtls_pkcs7::GetDeps(second);
    if first_deps != second_deps {
        return -1;
    }
    let mut first_ptr = first;
    let mut second_ptr = second;
    for _ in 0..first_deps {
        unsafe {
            if (*first_ptr).oid.tag != (*second_ptr).oid.tag ||
                (*first_ptr).oid.len != (*second_ptr).oid.len ||
                libc::memcmp((*first_ptr).oid.p as *const _, (*second_ptr).oid.p as *const _, (*second_ptr).oid.len as usize) != 0 ||
                (*first_ptr).private_next_merged != (*second_ptr).private_next_merged ||
                (*first_ptr).val.len != (*second_ptr).val.len {
                return -1;
            }
        }
        unsafe {
            if crate::src_mbedtls_pkcs7::CompareX509String(&(*first_ptr).val, &(*second_ptr).val) != 0 {
                return -1;
            }
            first_ptr = (*first_ptr).next;
            second_ptr = (*second_ptr).next;
        }
    }
    0
}