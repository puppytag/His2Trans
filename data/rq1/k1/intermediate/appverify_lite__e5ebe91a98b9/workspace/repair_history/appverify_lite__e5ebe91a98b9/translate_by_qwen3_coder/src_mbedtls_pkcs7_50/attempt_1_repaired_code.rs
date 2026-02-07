fn DLogCrtVerifyInfo(flags: u32) {
    let mut vrfyBuf: [::core::ffi::c_char; 512] = [0; 512];
    unsafe {
        let _ = crate::compat::memset_s(
            vrfyBuf.as_mut_ptr() as *mut ::core::ffi::c_void,
            512,
            0,
            512,
        );
        crate::compat::mbedtls_x509_crt_verify_info(
            vrfyBuf.as_mut_ptr(),
            512,
            " ! \0".as_ptr() as *const ::core::ffi::c_char,
            flags,
        );
        let _ = crate::compat::HiLogPrint(
            crate::types::LOG_CORE,
            crate::types::LOG_DEBUG,
            0xD001100,
            "appverify\0".as_ptr() as *const ::core::ffi::c_char,
            "[%s:%d]: %s\0".as_ptr() as *const ::core::ffi::c_char,
            "DLogCrtVerifyInfo\0".as_ptr() as *const ::core::ffi::c_char,
            981,
            vrfyBuf.as_ptr(),
        );
    }
}