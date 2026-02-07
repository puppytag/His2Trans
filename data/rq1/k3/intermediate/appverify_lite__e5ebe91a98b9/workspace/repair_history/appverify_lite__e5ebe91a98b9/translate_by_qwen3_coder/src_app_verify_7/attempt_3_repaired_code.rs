pub extern "C" fn GetHashUnitLen(hashAlg: i32) -> i32 {
    let _ = HiLogPrint(
        LOG_CORE,
        LOG_INFO,
        0xD001100,
        "appverify\0".as_ptr() as *const i8,
        "[%s:%d]: algId: %d\0".as_ptr() as *const i8,
        __FUNCTION__!(),
        247,
        hashAlg,
    );
    unsafe {
        let info = mbedtls_md_info_from_type(hashAlg as mbedtls_md_type_t);
        if info.is_null() {
            return 0;
        }
        mbedtls_md_get_size(info) as i32
    }
}