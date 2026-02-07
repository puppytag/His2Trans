pub extern "C" fn GetHashUnitLen(hashAlg: i32) -> i32 {
    let _ = crate::compat::HiLogPrint(
        crate::types::LOG_CORE,
        crate::types::LOG_INFO,
        0xD001100,
        "appverify\0".as_ptr() as *const i8,
        "[%s:%d]: algId: %d\0".as_ptr() as *const i8,
        __FUNCTION__!(),
        247,
        hashAlg,
    );
    let md_info = unsafe { crate::compat::mbedtls_md_info_from_type(hashAlg as crate::types::mbedtls_md_type_t) };
    unsafe { crate::compat::mbedtls_md_get_size(md_info) as i32 }
}