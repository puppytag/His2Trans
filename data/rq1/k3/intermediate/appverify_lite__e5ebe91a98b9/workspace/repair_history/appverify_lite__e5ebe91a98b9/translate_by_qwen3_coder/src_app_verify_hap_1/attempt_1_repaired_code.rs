pub extern "C" fn GetDigestAlgorithmId(signAlgorithm: u32) -> i32 {
    match signAlgorithm & crate::types::ALGORITHM_MASK {
        crate::types::ALGORITHM_SHA256 | crate::types::ALGORITHM_PKCS1_SHA256 => crate::types::MBEDTLS_MD_SHA256 as i32,
        crate::types::ALGORITHM_SHA384 | crate::types::ALGORITHM_PKCS1_SHA384 => crate::types::MBEDTLS_MD_SHA384 as i32,
        crate::types::ALGORITHM_SHA512 | crate::types::ALGORITHM_PKCS1_SHA512 => crate::types::MBEDTLS_MD_SHA512 as i32,
        _ => {
            unsafe {
                let _ = crate::compat::HiLogPrint(
                    crate::types::LOG_CORE,
                    crate::types::LOG_ERROR,
                    0xD001100,
                    b"appverify\0".as_ptr() as *const _,
                    b"[%s:%d]: signAlgorithm: %u error\0".as_ptr() as *const _,
                    b"GetDigestAlgorithmId\0".as_ptr() as *const _,
                    38,
                    signAlgorithm,
                );
            }
            crate::types::V_ERR as i32
        }
    }
}