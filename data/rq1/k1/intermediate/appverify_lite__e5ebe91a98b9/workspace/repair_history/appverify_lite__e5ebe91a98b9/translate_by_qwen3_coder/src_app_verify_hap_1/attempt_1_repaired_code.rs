pub extern "C" fn GetDigestAlgorithmId(signAlgorithm: u32) -> i32 {
    match signAlgorithm & ALGORITHM_MASK {
        ALGORITHM_SHA256 | ALGORITHM_PKCS1_SHA256 => MBEDTLS_MD_SHA256 as i32,
        ALGORITHM_SHA384 | ALGORITHM_PKCS1_SHA384 => MBEDTLS_MD_SHA384 as i32,
        ALGORITHM_SHA512 | ALGORITHM_PKCS1_SHA512 => MBEDTLS_MD_SHA512 as i32,
        _ => {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    LOG_CORE,
                    LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: signAlgorithm: %u error\0".as_ptr() as *const _,
                    b"GetDigestAlgorithmId\0".as_ptr() as *const _,
                    38,
                    signAlgorithm,
                );
            }
            V_ERR as i32
        }
    }
}